// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! The left rail: brand mark, the seven navigation destinations, the saved
//! playlists and Settings pinned to the foot of the panel. `nav_row`,
//! `plist_row` and `settings_row` are the one geometry source, so every
//! hit-test lands on exactly the pill the painter drew.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use crate::ui::art::cover;
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{cap_h, pill, ITEM, LABEL, SECONDARY, S1, S2, S3, S4, S5, S6, SECTION};
use crate::ui::paint::{fill, stroke, text, text_mid, text_right};
use crate::ui::state::{View, NAV, PLAYLISTS};
use crate::ui::text::{count, truncate_to_width, upper};
use crate::ui::theme::{CYAN, CYAN_WASH, EDGE, INK, MID, MUTE, PANEL, VIOLET, VIOLET_WASH};

pub const ROW_H: i32 = 42;
pub const PLIST_H: i32 = 44;
const BRAND_H: i32 = 68;
const GLYPHS: [Glyph; 7] = [
    Glyph::Home,
    Glyph::Grid,
    Glyph::Compass,
    Glyph::Search,
    Glyph::Radio,
    Glyph::Note,
    Glyph::Download,
];

pub fn nav_row(r: Rect, i: usize) -> Rect {
    Rect::new(r.x + S3, r.y + BRAND_H + i as i32 * (ROW_H + S1), r.w - S3 * 2, ROW_H)
}

fn nav_bottom(r: Rect) -> i32 {
    nav_row(r, NAV.len() - 1).bottom()
}

pub fn plist_visible(r: Rect) -> usize {
    let top = nav_bottom(r) + S5 + line_head();
    let room = settings_row(r).y - S5 - top;
    (room / PLIST_H).clamp(0, PLAYLISTS.len() as i32) as usize
}

pub fn plist_row(r: Rect, i: usize) -> Rect {
    let top = nav_bottom(r) + S5 + line_head();
    Rect::new(r.x + S3, top + i as i32 * PLIST_H, r.w - S3 * 2, PLIST_H)
}

fn line_head() -> i32 {
    S5 + cap_h(LABEL) * 2
}

pub fn settings_row(r: Rect) -> Rect {
    Rect::new(r.x + S3, r.bottom() - S5 - ROW_H, r.w - S3 * 2, ROW_H)
}

pub fn row_at(r: Rect, x: i32, y: i32) -> Option<View> {
    if settings_row(r).contains(x, y) {
        return Some(View::Settings);
    }
    (0..NAV.len()).find(|&i| nav_row(r, i).contains(x, y)).map(|i| NAV[i].0)
}

pub fn plist_at(r: Rect, x: i32, y: i32) -> Option<usize> {
    (0..plist_visible(r)).find(|&i| plist_row(r, i).contains(x, y))
}

fn entry(fb: &mut PaintBuffer, icons: &Icons, r: Rect, label: &str, g: Glyph, on: bool) {
    if on {
        fill(fb, r, pill(r.h), CYAN_WASH);
        stroke(fb, r, pill(r.h), 1, EDGE);
    }
    let ink = if on { INK } else { MID };
    icons.centred(fb, Rect::new(r.x + S3, r.y, 30, r.h), 19, g, ink);
    let tx = S3 + 30 + S2;
    text_mid(fb, Rect::new(r.x + tx, r.y, r.w - tx, r.h), label, ink, ITEM);
}

fn playlist(fb: &mut PaintBuffer, r: Rect, name: &str, n: u32, on: bool) {
    if on {
        fill(fb, r, pill(r.h) / 2, VIOLET_WASH);
        stroke(fb, r, pill(r.h) / 2, 1, VIOLET);
    }
    let art = Rect::new(r.x + S2, r.cy() - 17, 34, 34);
    cover(fb, art, name, 8);
    let tx = art.right() + S3;
    let tw = r.right() - S3 - tx;
    let name = truncate_to_width(name, SECONDARY, tw);
    text(fb, tx, r.cy() - cap_h(SECONDARY) - S1, &name, INK, SECONDARY);
    text(fb, tx, r.cy() + S1, &count(n as usize, "track", "tracks"), MUTE, LABEL);
}

pub fn sidebar(fb: &mut PaintBuffer, icons: &Icons, r: Rect, view: View, sel: Option<usize>) {
    fill(fb, r, 0, PANEL);
    fill(fb, Rect::new(r.right() - 1, r.y, 1, r.h), 0, EDGE);

    let mark = Rect::new(r.x + S5, r.y + S6, 30, 30);
    fill(fb, mark, 9, VIOLET);
    fill(fb, mark.inset(5), 5, CYAN);
    text(fb, mark.right() + S3, mark.cy() - cap_h(SECTION), "Resonare", INK, SECTION);

    for (i, (v, label)) in NAV.iter().enumerate() {
        entry(fb, icons, nav_row(r, i), label, GLYPHS[i], *v == view);
    }

    let sep_y = nav_bottom(r) + S4;
    fill(fb, Rect::new(r.x + S5, sep_y, r.w - S5 * 2, 1), 0, EDGE);

    let shown = plist_visible(r);
    if shown > 0 {
        let head = Rect::new(r.x + S5, sep_y + S4, r.w - S5 * 2, cap_h(LABEL) * 2);
        text(fb, head.x, head.y, &upper("Your playlists"), MUTE, LABEL);
        text_right(fb, head, "+", MUTE, LABEL);
    }

    for (i, (name, n)) in PLAYLISTS.iter().take(shown).enumerate() {
        playlist(fb, plist_row(r, i), name, *n, sel == Some(i));
    }

    let foot = settings_row(r);
    fill(fb, Rect::new(r.x + S5, foot.y - S4, r.w - S5 * 2, 1), 0, EDGE);
    entry(fb, icons, foot, "Settings", Glyph::Gear, view == View::Settings);
}
