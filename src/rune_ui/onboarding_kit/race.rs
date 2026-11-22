//! One note, written twice, side by side, one after the other.
//!
//! First, by hand: the scene's keystrokes type the note on the left, typo,
//! backspace and all, on a small keyboard whose keys press as they're
//! struck, until it sends and shows how long it took. The right-hand note
//! waits, dark, and nothing on it lights.
//!
//! Then Utter: `⌥` goes down, the keys fade, a listening pill rises and the
//! words land as they're spoken, with the time that took beside the typing's.
//!
//! The scene's own clock ([`super::versus::scene_time`]) drives the typing,
//! so its sounds, its Skip and its length stay the scene's; the step after
//! it ([`Step::YourTurn`] on) drives the speaking.

use rune::{El, Shadow, TextStyle, col, el, row, text};

use super::super::controls::Paint;
use super::composition::{STAGE_H, STAGE_W};
use super::keyboard::ember;
use super::stories::{Onboard, Step};
use super::typing::Who;
use super::versus::{scene_time, take};

const GAP: f32 = 18.0;
const DOC_W: f32 = (STAGE_W - GAP) * 0.5;
const DOC_H: f32 = 268.0;
const DOC_PAD: f32 = 16.0;
const BAR_H: f32 = 38.0;
const FOOT_H: f32 = 40.0;
const BODY_FONT: TextStyle = TextStyle::new(15.0, 450.0).lh(22.0);
const SMALL: TextStyle = TextStyle::new(12.0, 600.0).lh(16.0);

const KEY_W: f32 = 21.0;
const KEY_H: f32 = 25.0;
const KEY_GAP: f32 = 3.5;
const KEY_ROWS: [&str; 3] = ["QWERTYUIOP", "ASDFGHJKL", "ZXCVBNM,."];
pub(super) const KB_W: f32 = 10.0 * KEY_W + 9.0 * KEY_GAP;
pub(super) const KB_H: f32 = 4.0 * KEY_H + 3.0 * KEY_GAP;
const KB_TOP: f32 = DOC_H + 22.0;

/// What each side shows now.
struct Race {
    typed: String,
    typed_secs: f64,
    typed_done: bool,
    spoken: String,
    /// The speaking clock has started.
    spoken_started: bool,
    spoken_secs: f64,
    spoken_done: bool,
    /// It's Utter's turn.
    live: bool,
}

pub(super) fn stage(p: Paint, s: &Onboard) -> El {
    let race = race(s);
    let left_key = |id: &str| match scene_time(s) {
        Some(t) if !race.live => s.scene.depth(t, id),
        _ => 0.0,
    };
    // Nothing on Utter's side lights until it's listening.
    let hearing = s.step == Step::Hearing;
    let right_lit = |id: &str| f32::from(id == "option" && hearing);

    let left = document(
        p,
        "by-hand",
        Doc {
            label: "By hand",
            accent: false,
            idle: false,
            body: &race.typed,
            placeholder: "Start typing…",
            caret: !race.typed_done && race.typed_secs > 0.0,
            secs: Some(race.typed_secs),
            done: race.typed_done,
            foot: race
                .typed_done
                .then(|| format!("Sent in {:.1} s", race.typed_secs)),
            winner: false,
            now: s.now,
        },
    );
    let right = document(
        p,
        "with-utter",
        Doc {
            label: "With Utter",
            accent: true,
            idle: !race.live,
            body: &race.spoken,
            placeholder: if race.live {
                "Hold ⌥ and say it…"
            } else {
                "Next, just say it…"
            },
            caret: false,
            secs: race.spoken_started.then_some(race.spoken_secs),
            done: race.spoken_done,
            foot: race.spoken_done.then(|| {
                let faster = race.typed_secs - race.spoken_secs;
                if faster >= 0.1 {
                    format!("Said in {:.1} s · {:.1} s faster", race.spoken_secs, faster)
                } else {
                    format!("Said in {:.1} s", race.spoken_secs)
                }
            }),
            winner: hearing || race.spoken_done,
            now: s.now,
        },
    )
    .child(listening(p, hearing, s.now));

    el().key("stage")
        .size(STAGE_W, STAGE_H)
        .child(el().key("l").absolute(0.0, 0.0).size(DOC_W, DOC_H).child(left))
        .child(
            el().key("r")
                .absolute(DOC_W + GAP, 0.0)
                .size(DOC_W, DOC_H)
                .child(right),
        )
        .child(
            el().key("kb-l")
                .absolute((DOC_W - KB_W) * 0.5, KB_TOP)
                .size(KB_W, KB_H)
                .child(keyboard(p, "kb-l", p.k.c.bg_deep, &left_key, 0.0)),
        )
        .child(
            el().key("kb-r")
                .absolute(DOC_W + GAP + (DOC_W - KB_W) * 0.5, KB_TOP)
                .size(KB_W, KB_H)
                .child(keyboard(
                    p,
                    "kb-r",
                    p.k.c.bg_deep,
                    &right_lit,
                    if hearing { 0.72 } else { 0.0 },
                )),
        )
        .child(caption(
            p,
            "cap-l",
            (DOC_W - KB_W) * 0.5,
            if race.typed_done {
                "…finally".to_string()
            } else {
                "one key at a time".to_string()
            },
        ))
        .child(caption(
            p,
            "cap-r",
            DOC_W + GAP + (DOC_W - KB_W) * 0.5,
            if hearing {
                "⌥ held · talking".to_string()
            } else if race.spoken_done {
                "released · done".to_string()
            } else {
                "hold ⌥ and talk".to_string()
            },
        ))
}

fn race(s: &Onboard) -> Race {
    let live = s.step >= Step::YourTurn;
    let scene = &s.scene;
    let first = scene.strokes().first().map_or(0.0, |k| k.at);
    let sent = scene.lines().iter().find(|l| l.who == Who::You);
    let sent_at = sent.map_or(f64::MAX, |l| l.at);
    let note = sent.map_or("", |l| l.text.as_str());
    let t = scene_time(s).unwrap_or(f64::MIN);

    let typed_done = live || t >= sent_at;
    let typed = if typed_done {
        note.to_string()
    } else {
        scene.draft(t).0.to_string()
    };
    let typed_secs = if live {
        sent_at - first
    } else {
        (t.min(sent_at) - first).clamp(0.0, sent_at - first)
    };

    // The speaking clock is the take's own: from the shortcut going down to
    // the last word.
    let spoken_started = live && s.take_from.is_some();
    let spoken_secs = s
        .take_from
        .filter(|_| live)
        .map_or(0.0, |from| (s.now - from).clamp(0.0, take().release));
    Race {
        typed,
        typed_secs,
        typed_done,
        spoken: if live { s.partial.to_string() } else { String::new() },
        spoken_started,
        spoken_secs,
        spoken_done: matches!(s.step, Step::Heard | Step::Done),
        live,
    }
}

struct Doc<'a> {
    label: &'a str,
    accent: bool,
    /// Waiting its turn: nothing on it lights.
    idle: bool,
    body: &'a str,
    placeholder: &'a str,
    caret: bool,
    secs: Option<f64>,
    done: bool,
    foot: Option<String>,
    winner: bool,
    now: f64,
}

fn document(p: Paint, key: &str, d: Doc<'_>) -> El {
    let c = &p.k.c;
    let warm = ember(p);
    let ink = if d.accent && !d.idle { warm } else { p.muted() };
    let blink = (d.now * 1.7).fract() < 0.55;
    let shown = if d.body.is_empty() {
        d.placeholder.to_string()
    } else if d.caret && blink {
        format!("{}\u{258F}", d.body)
    } else {
        d.body.to_string()
    };
    let body_ink = if d.body.is_empty() { p.faint() } else { p.text() };

    let timer = d.secs.map(|secs| {
        let (fill, rim, label) = if d.done {
            (warm.alpha(0.18), warm.alpha(0.45), warm)
        } else {
            (c.bg_deep.alpha(0.5), c.border_weak, p.muted())
        };
        row()
            .key("timer")
            .px(9.0)
            .h(22.0)
            .min_w(64.0)
            .center()
            .radius(11.0)
            .fill(fill)
            .border(p.hair, rim)
            .child(text(format!("{secs:.1} s")).key("t").font(SMALL).color(label))
    });

    let bar = row()
        .key("bar")
        .w_full()
        .h(BAR_H)
        .px(DOC_PAD)
        .center_y()
        .justify_between()
        .child(text(d.label).key("label").font(SMALL).color(ink))
        .children(timer);

    let foot = row()
        .key("foot")
        .w_full()
        .h(FOOT_H)
        .px(DOC_PAD)
        .center_y()
        .children(d.foot.map(|line| {
            text(line)
                .key("foot-text")
                .font(TextStyle::new(14.0, 650.0).lh(18.0))
                .color(if d.accent { warm } else { p.muted() })
                .transition(rune::Transition::opacity())
        }));

    let rim = if d.winner { warm.alpha(0.7) } else { c.border_base };
    el().key(key.to_string())
        .size(DOC_W, DOC_H)
        .radius(16.0)
        .continuous_corners()
        .fill(c.layer_02)
        .border(p.hair, rim)
        .box_shadow(Shadow {
            x: 0.0,
            y: 8.0,
            blur: 24.0,
            spread: -6.0,
            color: if d.winner {
                warm.alpha(0.35)
            } else {
                c.bg_deep.alpha(0.35)
            },
            inset: false,
        })
        .child(
            col()
                .key("page")
                .w_full()
                .h_full()
                .child(bar)
                .child(
                    col()
                        .key("body")
                        .grow(1.0)
                        .w_full()
                        .px(DOC_PAD)
                        .child(
                            text(shown)
                                .key("copy")
                                .font(BODY_FONT)
                                .color(body_ink)
                                .w(DOC_W - 2.0 * DOC_PAD)
                                .wrap()
                                .max_lines(8),
                        ),
                )
                .child(foot),
        )
}

/// The pill that rises in the corner of the spoken note while Utter listens.
fn listening(p: Paint, on: bool, now: f64) -> El {
    let warm = ember(p);
    let bars = (0..5).map(|i| {
        let wave = ((now * 11.0 + i as f64 * 1.7).sin() * (now * 3.1 + i as f64).sin()).abs();
        let h = if on { 4.0 + 12.0 * wave as f32 } else { 4.0 };
        el().key(format!("bar-{i}"))
            .w(3.0)
            .h(h)
            .radius(1.5)
            .fill(warm)
            .instant_changes()
    });
    el().key("listening")
        .absolute(DOC_W - 112.0 - DOC_PAD, DOC_H - FOOT_H + 7.0)
        .child(
            row()
                .key("pill")
                .gap(8.0)
                .px(10.0)
                .h(26.0)
                .center_y()
                .radius(13.0)
                .fill(warm.alpha(0.16))
                .border(p.hair, warm.alpha(0.4))
                .opacity(if on { 1.0 } else { 0.0 })
                .transition(rune::Transition::opacity())
                .child(row().key("level").gap(2.5).h(18.0).center_y().children(bars))
                .child(text("Listening").key("word").font(SMALL).color(warm)),
        )
}

fn caption(p: Paint, key: &str, x: f32, copy: String) -> El {
    el().key(key.to_string())
        .absolute(x, KB_TOP + KB_H + 12.0)
        .w(KB_W)
        .child(
            row()
                .key("line")
                .w_full()
                .center()
                .child(text(copy).key("copy").font(SMALL).color(p.faint())),
        )
}

/// A small keyboard: three rows of letters, backspace, and `⌥` over a space
/// bar. `pressed` is how far each key is down and `fade` how far every key
/// but a lit `⌥` has gone dim.
pub(super) fn keyboard(
    p: Paint,
    key: &str,
    ground: rune::Color,
    pressed: &dyn Fn(&str) -> f32,
    fade: f32,
) -> El {
    let c = &p.k.c;
    let warm = ember(p);
    let base = c.surface_raised.mix(c.layer_02, 0.32);
    let lit_cap = warm;
    let ink = c.text_base;

    let mut caps: Vec<El> = Vec::with_capacity(36);
    let mut cap = |id: &str, legend: &str, x: f32, y: f32, w: f32| {
        let down = pressed(id).clamp(0.0, 1.0);
        let is_option = id == "option";
        let vis = if is_option { 1.0 } else { 1.0 - fade };
        let fill = ground.mix(base.mix(lit_cap, down), vis);
        let rim = ground.mix(c.border_weak.mix(lit_cap, down * 0.8), vis);
        let ink = ground.mix(ink.mix(c.bg_deep, down * 0.9), vis);
        let mut el = el()
            .key(format!("cap-{id}"))
            .absolute(x, y + 2.0 * down)
            .size(w, KEY_H)
            .radius(6.0)
            .continuous_corners()
            .fill(fill)
            .border(p.hair, rim)
            .instant_changes();
        if down > 0.05 {
            el = el.box_shadow(Shadow {
                x: 0.0,
                y: 1.0,
                blur: 12.0,
                spread: 0.5,
                color: warm.alpha(0.45 * down),
                inset: false,
            });
        }
        let pt = if is_option { 13.0 } else { 11.0 };
        caps.push(el.center().child(
            text(legend)
                .key("lbl")
                .font(TextStyle::new(pt, if down > 0.2 { 650.0 } else { 500.0 }).lh(pt + 2.0))
                .color(ink),
        ));
    };

    for (r, letters) in KEY_ROWS.iter().enumerate() {
        let count = letters.chars().count() as f32 + if r == 2 { 1.0 } else { 0.0 };
        let width = count * KEY_W + (count - 1.0) * KEY_GAP;
        let x0 = (KB_W - width) * 0.5;
        let y = r as f32 * (KEY_H + KEY_GAP);
        for (i, ch) in letters.chars().enumerate() {
            let id = ch.to_string();
            cap(&id, &id, x0 + i as f32 * (KEY_W + KEY_GAP), y, KEY_W);
        }
        if r == 2 {
            let i = letters.chars().count() as f32;
            cap("delete", "⌫", x0 + i * (KEY_W + KEY_GAP), y, KEY_W);
        }
    }
    let y = 3.0 * (KEY_H + KEY_GAP);
    let option_w = 2.6 * KEY_W;
    let space_w = 6.4 * KEY_W;
    let x0 = (KB_W - (option_w + KEY_GAP + space_w)) * 0.5;
    cap("option", "⌥", x0, y, option_w);
    cap("space", "", x0 + option_w + KEY_GAP, y, space_w);

    el().key(key.to_string())
        .size(KB_W, KB_H)
        .instant_changes()
        .children(caps)
}
