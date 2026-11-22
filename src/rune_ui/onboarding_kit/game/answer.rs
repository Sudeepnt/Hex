//! What the practice step draws: one question at a time, a big answer box
//! that fills with your words as you speak, progress dots, the small
//! keyboard with the shortcut lit, and a closing card.
//!
//! The game around it (dictation, the live words, the paste, Done) is
//! unchanged; this only decides how a round looks.

use rune::{Anim, El, Shadow, TextStyle, Transition, col, el, icon, row, text};
use rune_kit::{Button, ButtonVariant, glyphs};

use super::super::super::controls::{Paint, PushSize};
use super::super::cards::Notice;
use super::super::keyboard::ember;
use super::super::race::{KB_H, KB_W, keyboard};
use super::{Card, Look, Play, Round, STAGE_W, act, smooth};
use crate::desktop::i18n::tr;

const COLUMN_W: f32 = 600.0;
const COLUMN_X: f32 = (STAGE_W - COLUMN_W) * 0.5;
const DOTS_Y: f32 = 96.0;
const ASK_Y: f32 = 120.0;
const ASK_H: f32 = 112.0;
const ASK_PAD: f32 = 20.0;
const AVATAR: f32 = 44.0;
const REPLY_Y: f32 = ASK_Y + ASK_H + 14.0;
const REPLY_H: f32 = 152.0;
const REPLY_PAD: f32 = 20.0;
const KB_Y: f32 = REPLY_Y + REPLY_H + 22.0;
const ASK_FONT: TextStyle = TextStyle::new(22.0, 580.0).lh(29.0);
const SAY_FONT: TextStyle = TextStyle::new(21.0, 480.0).lh(29.0);
const SMALL: TextStyle = TextStyle::new(12.0, 650.0).lh(16.0);
/// How long an answered round stays up, showing it went, before the next
/// question takes its place.
const SENT_HOLD: f64 = 0.8;

/// The messages after the opening two, and the closing card, on `g`'s clock
/// `t`.
pub(super) fn view<S: 'static>(
    p: Paint,
    g: &Play,
    t: f64,
    reduced: bool,
    look: &Look<'_, S>,
) -> Vec<El> {
    let mut out = Vec::new();
    let ended = g.share.is_some_and(|at| t >= at);
    if !ended && let Some(round) = active(g, t) {
        out.push(dots(p, g, round));
        out.push(ask(p, round, reduced));
        out.push(reply::<S>(p, g, round, t, reduced));
        out.push(small_keyboard(p, g, round, look));
    }
    if let Some(at) = g.share
        && t >= at
    {
        out.push(closing(p, t - at, look));
    }
    out
}

/// The round on screen: the first unanswered one, or the one just answered
/// while its reply is still shown going.
fn active(g: &Play, t: f64) -> Option<&Round> {
    g.rounds
        .iter()
        .find(|r| !r.answered || t < r.shatter() + SENT_HOLD)
}

fn notice(r: &Round) -> Option<&Notice> {
    match &r.spec.card {
        Card::Chat(n) => Some(n),
        Card::Certificate => None,
    }
}

fn is_prompt(n: &Notice) -> bool {
    n.key == "prompt"
}

fn dots(p: Paint, g: &Play, round: &Round) -> El {
    let warm = ember(p);
    let total = super::OPENING.len();
    let answered = g.rounds.iter().filter(|r| r.answered).count() + g.dropped;
    let dots = (0..total).map(|i| {
        let (w, fill) = if i == round.n && !round.answered {
            (22.0, warm)
        } else if i < answered {
            (8.0, warm)
        } else {
            (8.0, p.k.c.border_base)
        };
        el().key(format!("dot-{i}"))
            .w(w)
            .h(8.0)
            .radius(4.0)
            .fill(fill)
    });
    row()
        .key("dots")
        .absolute(0.0, DOTS_Y)
        .w(STAGE_W)
        .gap(8.0)
        .center()
        .children(dots)
}

fn ask(p: Paint, round: &Round, reduced: bool) -> El {
    let c = &p.k.c;
    let Some(n) = notice(round) else {
        return el().key("ask-none");
    };
    let prompt = is_prompt(n);
    let (tint, glyph) = if prompt {
        (ember(p), "✦".to_string())
    } else {
        let name = if n.from == "Mom" { "coral" } else { "cobalt" };
        (
            utter_theme::scale(name, p.k.dark, if p.k.dark { 7 } else { 6 }),
            n.from.chars().next().map(String::from).unwrap_or_default(),
        )
    };
    let avatar = el()
        .key("avatar")
        .size(AVATAR, AVATAR)
        .radius(13.0)
        .continuous_corners()
        .fill(tint)
        .center()
        .child(
            text(glyph)
                .key("g")
                .font(TextStyle::new(19.0, 750.0).lh(22.0))
                .color(c.white),
        );
    let meta = row()
        .key("meta")
        .gap(8.0)
        .child(
            text(n.from)
                .key("from")
                .font(TextStyle::new(13.0, 700.0).lh(17.0))
                .color(p.text()),
        )
        .child(
            text(n.place.unwrap_or("Messages"))
                .key("where")
                .font(TextStyle::new(13.0, 550.0).lh(17.0))
                .color(p.muted()),
        );
    let enter = if reduced {
        Transition::opacity()
    } else {
        Transition::opacity().combined(Transition::offset(0.0, 16.0))
    };
    el().key(format!("ask-{}", round.n))
        .absolute(COLUMN_X, ASK_Y)
        .size(COLUMN_W, ASK_H)
        .transition(enter)
        .radius(20.0)
        .continuous_corners()
        .fill(if prompt {
            c.layer_02.mix(ember(p), 0.07)
        } else {
            c.layer_02
        })
        .border(p.hair, if prompt { ember(p).alpha(0.4) } else { c.border_base })
        .box_shadow(Shadow {
            x: 0.0,
            y: 12.0,
            blur: 32.0,
            spread: -8.0,
            color: c.bg_deep.alpha(0.5),
            inset: false,
        })
        .child(
            row()
                .key("inner")
                .w_full()
                .h_full()
                .px(ASK_PAD)
                .py(ASK_PAD)
                .gap(16.0)
                .items_start()
                .child(avatar)
                .child(
                    col()
                        .key("words")
                        .gap(5.0)
                        .grow(1.0)
                        .child(meta)
                        .child(
                            text(n.body)
                                .key("q")
                                .font(ASK_FONT)
                                .color(p.text())
                                .w(COLUMN_W - 2.0 * ASK_PAD - AVATAR - 16.0)
                                .wrap()
                                .max_lines(2),
                        ),
                ),
        )
}

const TALK: f32 = 72.0;

/// Starts or stops Utter's dictation as its shortcut would, for the on-screen
/// button. A press that never arrives (Linux and Windows) does nothing.
#[cfg(target_os = "macos")]
fn capture(action: crate::control::CaptureAction) {
    let Some(runtime) = crate::macos_rune::runtime() else {
        return;
    };
    let (reply, wait) = std::sync::mpsc::sync_channel(1);
    let control = crate::recognition::RecognitionControl::Capture {
        action,
        print: None,
        raw: false,
        reply,
    };
    if runtime.recognition(control).is_ok() {
        std::thread::spawn(move || {
            let _ = wait.recv_timeout(std::time::Duration::from_secs(3));
        });
    }
}

#[cfg(not(target_os = "macos"))]
fn capture(_: crate::control::CaptureAction) {}

/// The mic: press and hold to talk, let go to send. The shortcut does the same.
fn talk_button<S: 'static>(p: Paint, live: bool, t: f64, reduced: bool) -> El {
    let c = &p.k.c;
    let warm = ember(p);
    let pulse = if live && !reduced {
        1.0 + 0.045 * (t * 6.0).sin() as f32
    } else {
        1.0
    };
    let (fill, rim, ink) = if live {
        (warm, warm, c.bg_deep)
    } else {
        (warm.alpha(0.16), warm.alpha(0.6), warm)
    };
    let button = el()
        .key("talk")
        .size(TALK, TALK)
        .radius(TALK * 0.5)
        .fill(fill)
        .border(1.5, rim)
        .center()
        .scale(pulse)
        .scale_motion(Anim::INSTANT)
        .box_shadow(Shadow {
            x: 0.0,
            y: 0.0,
            blur: if live { 30.0 } else { 14.0 },
            spread: if live { 4.0 } else { 0.0 },
            color: warm.alpha(if live { 0.45 } else { 0.12 }),
            inset: false,
        })
        .child(icon(&glyphs::MIC, 30.0).key("mic").color(ink))
        .on_drag(|_: &mut S, drag: &mut rune::Drag, _| match drag.phase {
            rune::Phase::Down => capture(crate::control::CaptureAction::Start),
            rune::Phase::Up => capture(crate::control::CaptureAction::Stop),
            _ => {}
        });
    col()
        .key("talk-col")
        .w(TALK + 20.0)
        .gap(8.0)
        .items_center()
        .child(button)
        .child(
            text(if live { "Release to send" } else { "Hold to talk" })
                .key("talk-cap")
                .font(SMALL)
                .color(if live { warm } else { p.muted() }),
        )
}

fn reply<S: 'static>(p: Paint, g: &Play, round: &Round, t: f64, reduced: bool) -> El {
    let c = &p.k.c;
    let warm = ember(p);
    let prompt = notice(round).is_some_and(is_prompt);
    let live = g.holding && !round.answered;
    let blink = (t * 1.7).fract() < 0.55;
    let sent = round.answered;
    let noun = if prompt { "prompt" } else { "reply" };

    let heading = row()
        .key("label")
        .w_full()
        .justify_between()
        .child(
            text(if prompt { "YOUR PROMPT" } else { "YOUR REPLY" })
                .key("l")
                .font(SMALL)
                .color(if live || sent { warm } else { p.faint() }),
        )
        .child(
            text(if sent { "Sent ✓" } else { "" })
                .key("s")
                .font(SMALL)
                .color(warm)
                .transition(Transition::opacity()),
        );

    let width = COLUMN_W - 2.0 * REPLY_PAD - TALK - 20.0 - 18.0;
    let body = if round.text.is_empty() {
        let (title, sub) = if live {
            ("Listening…".to_string(), format!("Say your {noun}."))
        } else {
            (
                format!("Hold the mic, or hold ⌥, and say your {noun}."),
                "Let go when you're done.".to_string(),
            )
        };
        col()
            .key("how")
            .gap(6.0)
            .child(
                text(title)
                    .key("how-t")
                    .font(TextStyle::new(20.0, 620.0).lh(26.0))
                    .color(if live { warm } else { p.text() })
                    .w(width)
                    .wrap()
                    .max_lines(2),
            )
            .child(
                text(sub)
                    .key("how-s")
                    .font(TextStyle::new(15.0, 480.0).lh(21.0))
                    .color(p.muted()),
            )
    } else {
        let said = if live && blink {
            format!("{}\u{258F}", round.text)
        } else {
            round.text.clone()
        };
        col().key("said").child(
            text(said)
                .key("say")
                .font(SAY_FONT)
                .color(p.text())
                .w(width)
                .wrap()
                .max_lines(3),
        )
    };

    el().key(format!("reply-{}", round.n))
        .absolute(COLUMN_X, REPLY_Y)
        .size(COLUMN_W, REPLY_H)
        .radius(20.0)
        .continuous_corners()
        .fill(c.layer_02)
        .border(if live { 1.5 } else { p.hair }, if live { warm } else { c.border_base })
        .box_shadow(Shadow {
            x: 0.0,
            y: 0.0,
            blur: if live { 34.0 } else { 0.0 },
            spread: if live { 2.0 } else { 0.0 },
            color: warm.alpha(if live { 0.22 } else { 0.0 }),
            inset: false,
        })
        .child(
            row()
                .key("box")
                .w_full()
                .h_full()
                .px(REPLY_PAD)
                .py(REPLY_PAD)
                .gap(18.0)
                .items_center()
                .child(talk_button::<S>(p, live, t, reduced))
                .child(
                    col()
                        .key("right")
                        .grow(1.0)
                        .h_full()
                        .gap(10.0)
                        .child(heading)
                        .child(body),
                ),
        )
}

fn small_keyboard<S>(p: Paint, g: &Play, round: &Round, look: &Look<'_, S>) -> El {
    let held = g.holding && !round.answered;
    let option = look.caps.iter().any(|k| k == "⌥");
    let lit = |id: &str| f32::from(id == "option" && held && option);
    el().key("keyboard")
        .absolute((STAGE_W - KB_W) * 0.5, KB_Y)
        .size(KB_W, KB_H)
        .child(keyboard(
            p,
            "kb",
            p.k.c.bg_deep,
            &lit,
            if held && option { 0.72 } else { 0.0 },
        ))
}

/// The card after the last answer.
fn closing<S: 'static>(p: Paint, since: f64, look: &Look<'_, S>) -> El {
    let u = smooth(0.0, 0.5, since as f32);
    let warm = ember(p);
    let mark = row().key("mark").gap(5.0).h(54.0).items_end().children(
        [22.0_f32, 54.0, 34.0]
            .into_iter()
            .enumerate()
            .map(|(i, h)| {
                el().key(format!("m{i}"))
                    .w(14.0)
                    .h(h)
                    .radius(7.0)
                    .fill(warm)
            }),
    );
    let button = Button::new(tr("Continue"))
        .variant(ButtonVariant::Contrast)
        .on_press(act(&look.done))
        .build_push(p.k)
        .key("continue");
    col()
        .key("closing")
        .absolute(0.0, 0.0)
        .size(STAGE_W, super::STAGE_H)
        .center()
        .gap(14.0)
        .opacity(u)
        .opacity_motion(Anim::INSTANT)
        .child(mark)
        .child(
            text("That's Utter.")
                .key("h")
                .font(TextStyle::new(52.0, 780.0).lh(58.0).tracking(-1.2))
                .color(p.text()),
        )
        .child(
            text("Two replies and a prompt. No typing.")
                .key("sub")
                .font(TextStyle::new(18.0, 480.0).lh(24.0))
                .color(p.muted()),
        )
        .child(el().key("gap").h(10.0))
        .child(button)
}
