//! The Utter mark and the progress ring that becomes it.
//!
//! The mark is a rounded "u" holding a level bar (a lowercase u and a
//! microphone at once), drawn from the same 100×100 geometry as
//! `scripts/render-icons.swift`. While the speech model downloads, the mark's
//! place shows a round progress ring; once the download finishes and the ring
//! closes, the mark takes its place. `"Utter is ready"` builds the mark: the u
//! draws itself in one stroke, then the bar springs up inside it.
use rune::{El, LineCap, Shape, canvas};

use super::super::controls::Paint;

type Point = [f32; 2];

/// The mark's stroke as a fraction of its box, as in the app icon.
const MARK_STROKE: f32 = 0.13;
/// The mark's visual centre in its 100×100 box (its strokes span 11.5 to 86.5).
const MARK_MID_Y: f32 = 49.0;

/// The ring's stroke: thin from menu bar size up to the hero.
fn ring_stroke(size: f32) -> f32 {
    let t = ((size - 16.0) / 120.0).clamp(0.0, 1.0);
    1.2 + (2.2 - 1.2) * t
}

/// The u (an open path) and the bar (two points) around `centre`, `size` wide.
fn mark_paths(centre: Point, size: f32) -> (Vec<Point>, Vec<Point>) {
    let map = |x: f32, y: f32| {
        [
            centre[0] + (x - 50.0) / 100.0 * size,
            centre[1] + (y - MARK_MID_Y) / 100.0 * size,
        ]
    };
    let mut u = vec![map(24.0, 18.0), map(24.0, 54.0)];
    const ARC_STEPS: usize = 24;
    for i in 1..ARC_STEPS {
        // From the left stem round the bottom to the right one (y points down).
        let a = std::f32::consts::PI * (1.0 - i as f32 / ARC_STEPS as f32);
        u.push(map(50.0 + 26.0 * a.cos(), 54.0 + 26.0 * a.sin()));
    }
    u.push(map(76.0, 54.0));
    u.push(map(76.0, 18.0));
    let bar = vec![map(50.0, 24.0), map(50.0, 56.0)];
    (u, bar)
}

/// The part of the polyline `poly` its first `fraction` of length covers, and
/// where that part ends while it is still being drawn.
fn sweep_poly(poly: &[Point], fraction: f32) -> (Vec<Point>, Option<Point>) {
    let f = fraction.clamp(0.0, 1.0);
    if f <= 0.0 || poly.len() < 2 {
        return (Vec::new(), None);
    }
    if f >= 1.0 {
        return (poly.to_vec(), None);
    }
    let segs: Vec<f32> = poly
        .windows(2)
        .map(|w| (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]))
        .collect();
    let target = segs.iter().sum::<f32>() * f;
    let mut acc = 0.0;
    let mut out = vec![poly[0]];
    for (i, &len) in segs.iter().enumerate() {
        if acc + len >= target {
            let u = if len > 1e-5 {
                ((target - acc) / len).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let (a, b) = (poly[i], poly[i + 1]);
            let tip = [a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u];
            out.push(tip);
            return (out, Some(tip));
        }
        acc += len;
        out.push(poly[i + 1]);
    }
    let last = *out.last().unwrap_or(&poly[0]);
    (out, Some(last))
}

fn disc_points(centre: Point, radius: f32) -> Vec<Point> {
    (0..16)
        .map(|i| {
            let a = i as f32 / 16.0 * std::f32::consts::TAU;
            [centre[0] + radius * a.cos(), centre[1] + radius * a.sin()]
        })
        .collect()
}

/// A circle starting at twelve o'clock and running clockwise, ending where it
/// starts.
fn circle(centre: Point, radius: f32) -> Vec<Point> {
    const STEPS: usize = 72;
    (0..=STEPS)
        .map(|i| {
            let a = (i as f32 / STEPS as f32) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
            [centre[0] + radius * a.cos(), centre[1] + radius * a.sin()]
        })
        .collect()
}

pub(crate) fn utter_meter(p: Paint, key: &str, size: f32, progress: f32, closed: f32) -> El {
    canvas(meter_shape(p, size, progress, closed))
        .key(key)
        .size(size, size)
        .label(format!("{}%", (progress.clamp(0.0, 1.0) * 100.0).round()))
}

/// When each beat of the mark lands on `"Utter is ready"`, in seconds after
/// the page appears: the u starts, the u closes, the bar rises. Each sounds
/// one rising note.
pub(crate) const BUILD_AT: [f64; 3] = [0.0, 0.15, 0.30];
/// When the bar has settled.
pub(crate) const BUILD_SECS: f64 = 0.30 + SETTLE.0 * 1.5;
/// The bar's spring: duration and bounce.
const SETTLE: (f64, f64) = (0.36, 0.12);
/// How long the u takes to draw.
const DRAW: f64 = 0.30;
/// How long the bar takes to appear as it starts to rise.
const APPEAR: f64 = 0.08;

/// Progress of a critically-ish damped spring from 0 to 1 at `t` seconds.
fn settle(t: f64) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    let (duration, bounce) = SETTLE;
    let omega = std::f64::consts::TAU / duration;
    let zeta = 1.0 - bounce;
    let damped = omega * (1.0 - zeta * zeta).sqrt();
    let decay = (-zeta * omega * t).exp();
    (1.0 - decay * ((damped * t).cos() + zeta * omega / damped * (damped * t).sin())) as f32
}

/// Ease-out cubic over 0 to 1.
fn ease_out(t: f64) -> f32 {
    let t = t.clamp(0.0, 1.0);
    (1.0 - (1.0 - t).powi(3)) as f32
}

/// The Utter mark for `"Utter is ready"`, built `age` seconds after the page
/// appears: the u draws itself, then the bar springs up inside it. `handed`:
/// the mark arrives from the download pill already drawn. Reduced Motion
/// shows it built.
pub(crate) fn utter_logo(
    p: Paint,
    key: &str,
    size: f32,
    age: f64,
    reduced: bool,
    handed: bool,
) -> El {
    let pad = 12.0;
    let total = size + 2.0 * pad;
    let centre = [total * 0.5, total * 0.5];
    let line = MARK_STROKE * size;
    let (u, bar) = mark_paths(centre, size);

    let drawn = if reduced || handed {
        1.0
    } else {
        ease_out((age - BUILD_AT[0]) / DRAW)
    };
    let mut shape = Shape::new();
    let (stroke, tip) = sweep_poly(&u, drawn);
    if stroke.len() >= 2 {
        shape = shape.stroke_with(stroke, false, line, LineCap::Round, p.text());
    }
    if let Some(pt) = tip {
        shape = shape.fill(disc_points(pt, line * 0.5), p.text());
    }

    let t = age - BUILD_AT[2];
    let (rise, shown) = if reduced {
        (1.0, 1.0)
    } else {
        (settle(t), (t / APPEAR).clamp(0.0, 1.0) as f32)
    };
    if shown > 0.0 {
        let bottom = bar[1];
        let top = [bar[0][0], bottom[1] + (bar[0][1] - bottom[1]) * rise];
        shape = shape.stroke_with(
            vec![bottom, top],
            false,
            line,
            LineCap::Round,
            p.accent().alpha(shown),
        );
    }
    canvas(shape).key(key).size(total, total).label("Utter")
}

fn meter_shape(p: Paint, size: f32, progress: f32, closed: f32) -> Shape {
    let progress = progress.clamp(0.0, 1.0);
    let closed = closed.clamp(0.0, 1.0);
    let centre = [size * 0.5, size * 0.5];
    let lit = p.text();
    if progress >= 1.0 && closed >= 1.0 {
        // Done: the mark itself.
        let line = (MARK_STROKE * size).max(1.3);
        let (u, bar) = mark_paths(centre, size * 0.9);
        return Shape::new()
            .stroke_with(u, false, line, LineCap::Round, lit)
            .stroke_with(bar, false, line, LineCap::Round, p.accent());
    }
    let w = ring_stroke(size);
    let ring = circle(centre, size * 0.5 - w);
    let track = p.k.c.bg_deep.mix(p.k.c.border_base, 0.9);
    let mut shape = Shape::new().stroke_with(ring.clone(), true, w, LineCap::Round, track);
    if progress >= 1.0 {
        shape = shape.stroke_with(ring, true, w, LineCap::Round, lit);
    } else if progress > 0.002 {
        let (arc, tip) = sweep_poly(&ring, progress);
        shape = shape.stroke_with(arc, false, w, LineCap::Round, lit);
        if let Some(pt) = tip.filter(|_| size >= 36.0) {
            shape = shape.fill(disc_points(pt, w * 1.15), lit);
        }
    }
    shape
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stroke_stays_thin_from_menu_bar_to_hero() {
        assert!(ring_stroke(14.0) >= 1.1 && ring_stroke(14.0) <= 1.6);
        assert!(ring_stroke(136.0) <= 3.0);
    }

    #[test]
    fn the_mark_builds_and_settles() {
        assert_eq!(settle(-0.1), 0.0);
        let peak = (1..200)
            .map(|i| settle(i as f64 / 200.0))
            .fold(0.0, f32::max);
        assert!(
            peak > 1.0 && peak < 1.05,
            "a soft settle, not a bounce: {peak}"
        );
        assert!((settle(BUILD_SECS - BUILD_AT[2]) - 1.0).abs() < 0.01);
        assert!(
            BUILD_AT
                .windows(2)
                .all(|w| (0.12..=0.18).contains(&(w[1] - w[0])))
        );
        assert!(DRAW <= BUILD_AT[2], "the u is drawn before the bar rises");
    }

    #[test]
    fn the_mark_fits_its_box() {
        let size = 100.0;
        let (u, bar) = mark_paths([50.0, 50.0], size);
        let half = MARK_STROKE * size / 2.0;
        for [x, y] in u.iter().chain(bar.iter()) {
            assert!(*x - half >= 0.0 && *x + half <= size, "x {x}");
            assert!(*y - half >= 0.0 && *y + half <= size, "y {y}");
        }
    }

    #[test]
    fn sweep_covers_none_half_and_all_of_the_ring() {
        let ring = circle([50.0, 50.0], 40.0);
        assert!(sweep_poly(&ring, 0.0).0.is_empty());
        let (half, tip) = sweep_poly(&ring, 0.5);
        assert!(half.len() >= 3 && tip.is_some());
        let (full, tip) = sweep_poly(&ring, 1.0);
        assert_eq!(full.len(), ring.len());
        assert!(tip.is_none());
    }
}
