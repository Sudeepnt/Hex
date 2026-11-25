//! Settings: one sidebar item that holds Labs and Annotate (capture
//! builds, while Scrawl is there). Under the title bar a row of section tiles
//! (an icon badge, the section's name, a line on what it is, and whether
//! it's on) switches between them; each section below is the page it was.

use std::rc::Rc;

use rune::{El, Icon, Tx, col, icon, row, text};

use super::controls::{METRICS, Paint};
use super::{Pane, Ui, labs};
use crate::desktop::i18n::{tr, tr_fill};

pub(super) static GEAR_ICON: Icon = Icon {
    name: "utter-gear",
    view: 24.0,
    body: include_str!("../../assets/icons/gear.svg"),
};

/// A section of the Settings page.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) enum Section {
    #[default]
    Labs,
    #[cfg(utter_capture)]
    Annotate,
}

impl Section {
    fn key(self) -> &'static str {
        match self {
            #[cfg(utter_capture)]
            Section::Annotate => "settings-annotate",
            Section::Labs => "settings-labs",
        }
    }

    fn title(self) -> &'static str {
        match self {
            #[cfg(utter_capture)]
            Section::Annotate => tr("Annotate"),
            Section::Labs => tr("Labs"),
        }
    }

    fn about(self) -> &'static str {
        match self {
            #[cfg(utter_capture)]
            Section::Annotate => tr("Draw on your screen and talk over it"),
            Section::Labs => tr("Try features before they ship"),
        }
    }

    fn icon(self) -> &'static Icon {
        match self {
            #[cfg(utter_capture)]
            Section::Annotate => Pane::Annotate.icon(),
            Section::Labs => &labs::LABS_ICON,
        }
    }

    /// The tile's status chip, and whether it reads as on.
    fn status(self, ui: &Ui) -> (String, bool) {
        match self {
            #[cfg(utter_capture)]
            Section::Annotate => {
                let on = ui.settings.capture.enabled;
                ((if on { tr("On") } else { tr("Off") }).to_string(), on)
            }
            Section::Labs => {
                let on = labs::turned_on(ui);
                (tr_fill("{} on", &on.to_string()), on > 0)
            }
        }
    }
}

/// The sections this build and OS show, in the tiles' order.
fn sections(ui: &Ui) -> Vec<Section> {
    let mut all = vec![Section::Labs];
    #[cfg(utter_capture)]
    if Pane::Annotate.shown(ui) {
        all.push(Section::Annotate);
    }
    #[cfg(not(utter_capture))]
    let _ = ui;
    all
}

/// Whether the Settings page shows: Labs, or the Annotate pane.
pub(super) fn is_open(ui: &Ui) -> bool {
    #[cfg(utter_capture)]
    if ui.pane == Pane::Annotate {
        return true;
    }
    ui.labs_open
}

/// The section showing, while the page is open.
fn current(ui: &Ui) -> Section {
    #[cfg(utter_capture)]
    if ui.pane == Pane::Annotate && !ui.labs_open {
        return Section::Annotate;
    }
    Section::Labs
}

/// Shows `section`.
fn go(ui: &mut Ui, section: Section, tx: &mut Tx) {
    match section {
        #[cfg(utter_capture)]
        Section::Annotate => ui.select_pane(Pane::Annotate, tx),
        Section::Labs => labs::open(ui, tx),
    }
}

/// Opens the page on the section it showed last, or its first.
fn open(ui: &mut Ui, tx: &mut Tx) {
    if is_open(ui) {
        return;
    }
    let shown = sections(ui);
    let section = if shown.contains(&ui.settings_section) {
        ui.settings_section
    } else {
        shown[0]
    };
    go(ui, section, tx);
}

/// The sidebar's Settings item, under the panes.
pub(super) fn nav(ui: &Ui, p: Paint) -> El {
    super::sidebar::nav_entry(
        p,
        "nav-settings",
        &GEAR_ICON,
        tr("Advanced settings"),
        is_open(ui),
        Rc::new(open),
    )
}

/// The page under the title bar: the section tiles, then `body`, the
/// section's own page.
pub(super) fn page(ui: &Ui, p: Paint, body: El) -> El {
    col()
        .key("settings-page")
        .grow(1.0)
        .shrink()
        .min_h(0.0)
        .child(tiles(ui, p))
        .child(body)
}

fn tiles(ui: &Ui, p: Paint) -> El {
    let on = current(ui);
    row()
        .key("settings-sections")
        .role("tablist")
        .label(tr("Advanced settings"))
        .focus_group()
        .px(METRICS.gutter)
        .pt(2.0)
        .pb(14.0)
        .gap(10.0)
        .children(
            sections(ui)
                .into_iter()
                .map(|section| tile(ui, p, section, section == on)),
        )
}

const TILE_H: f32 = 62.0;
const BADGE: f32 = 36.0;

fn tile(ui: &Ui, p: Paint, section: Section, on: bool) -> El {
    let c = &p.k.c;
    let (status, lit) = section.status(ui);
    let badge = row()
        .key("badge")
        .size(BADGE, BADGE)
        .center()
        .radius(11.0)
        .fill(if on { c.icon_accent } else { c.layer_02 })
        .child(
            icon(section.icon(), 18.0)
                .key("icon")
                .color(if on { c.white } else { c.icon_muted }),
        );
    let words = col()
        .key("words")
        .grow(1.0)
        .shrink()
        .gap(1.0)
        .child(
            text(section.title())
                .key("title")
                .font(rune::TextStyle::new(14.0, 640.0).lh(19.0))
                .color(c.text_base)
                .truncate(),
        )
        .child(
            text(section.about())
                .key("about")
                .font(p.k.ty.small)
                .color(c.text_muted)
                .truncate(),
        );
    let chip = row()
        .key("status")
        .h(20.0)
        .px(8.0)
        .gap(5.0)
        .center_y()
        .radius(10.0)
        .fill(if lit {
            c.icon_accent.alpha(if p.k.dark { 0.18 } else { 0.12 })
        } else {
            c.layer_02
        })
        .child(
            super::controls::status_dot(if lit { c.icon_accent } else { c.text_faint })
                .size(6.0, 6.0)
                .radius(3.0),
        )
        .child(
            text(status)
                .key("label")
                .font(p.k.ty.small_medium)
                .color(if lit { c.text_base } else { c.text_muted }),
        );
    let mut tile = row()
        .key(section.key())
        .role("tab")
        .label(section.title())
        .description_static(section.about())
        .grow(1.0)
        .shrink()
        .min_w(0.0)
        .h(TILE_H)
        .px(12.0)
        .gap(12.0)
        .center_y()
        .radius(p.k.radius.lg)
        .focusable(true)
        .focus_selected(on)
        .focus_ring(2.0, 2.0, c.border_focus)
        .child(badge)
        .child(words)
        .child(chip);
    tile = if on {
        tile.fill(c.icon_accent.alpha(if p.k.dark { 0.13 } else { 0.07 }))
            .border(1.0, c.icon_accent.alpha(if p.k.dark { 0.55 } else { 0.45 }))
    } else {
        tile.fill(c.layer_01)
            .border(p.hair.max(0.5), c.border_muted)
            .hover(c.hover)
            .press(c.pressed)
            .on_press_tx(move |ui: &mut Ui, tx: &mut Tx| go(ui, section, tx))
    };
    tile
}
