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

use nonos_app_skeleton::PaintBuffer;

use super::icon_draw::draw;
use super::icon_path::Icon;
use super::theme::{INK, INK2, INK3, LINE};

// Shared line metrics for every non-Browse surface, so Recents, Search and Tags
// stack identically and a section label always advances by the same amount.
pub const LIST_ROW_H: u32 = 48;
pub const LABEL_ADV: u32 = 28;
const ROW_ICON: u32 = 20;
const TEXT_GAP: u32 = 16;
const TITLE_DY: u32 = 4;
const SUB_DY: u32 = 26;

// The two em sizes a row's text lines are drawn at, and the line box between
// them. `MIN_UI_PX` clamps both draw and measure, so anything measuring against
// these gets the same advance the painter did.
pub const TITLE_PX: f32 = 18.0;
pub const SUB_PX: f32 = 14.0;
pub const LINE_BOX: u32 = SUB_DY - TITLE_DY;

/// Where `screen_row` puts its text: the shared left edge, the title's top y,
/// and the sub line's. Anything drawn under those glyphs -- a search match band
/// -- measures from here rather than restating the offsets.
pub fn row_text(x: u32, y: u32) -> (u32, u32, u32) {
    (x + ROW_ICON + TEXT_GAP, y + TITLE_DY, y + SUB_DY)
}

/// One list line: filetype icon, title, its parent path beneath, and an
/// optional right-aligned meta column placed by `measure_ttf` rather than by
/// glyph count. Returns the height consumed so a column stacks without repeated
/// arithmetic.
pub fn screen_row(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    row: (&str, &str, &str),
    dir: bool,
    tint: u32,
) -> u32 {
    let (title, sub, meta) = row;
    let iy = y + (LIST_ROW_H - ROW_ICON) / 2;
    let kind = if dir { Icon::Folder } else { Icon::Doc };
    draw(fb, kind, x, iy, ROW_ICON, tint);
    let (tx, ty, sy) = row_text(x, y);
    let _ = fb.text_ttf(tx as i32, ty as i32, title, INK, TITLE_PX);
    let _ = fb.text_ttf(tx as i32, sy as i32, sub, INK3, SUB_PX);
    if !meta.is_empty() {
        let mw = fb.measure_ttf(meta, 14.0).max(0) as u32;
        let mx = (x + w).saturating_sub(mw);
        let _ = fb.text_ttf(mx as i32, (y + 4) as i32, meta, INK3, 14.0);
    }
    fb.fill_rect(x, y + LIST_ROW_H - 1, w, 1, LINE);
    LIST_ROW_H
}

/// A section heading in the label ink, returning its own advance.
pub fn section_label(fb: &mut PaintBuffer, x: u32, y: u32, text: &str) -> u32 {
    let _ = fb.text_ttf(x as i32, y as i32, text, INK3, 13.0);
    LABEL_ADV
}

/// A centred two-line honest empty state, measured so it stays centred at any
/// window width.
pub fn empty_state(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, title: &str, note: &str) {
    let iw = 40u32;
    draw(fb, Icon::Folder, x + w.saturating_sub(iw) / 2, y, iw, INK3);
    let tw = fb.measure_ttf(title, 20.0).max(0) as u32;
    let _ = fb.text_ttf((x + w.saturating_sub(tw) / 2) as i32, (y + 60) as i32, title, INK2, 20.0);
    let nw = fb.measure_ttf(note, 15.0).max(0) as u32;
    let _ = fb.text_ttf((x + w.saturating_sub(nw) / 2) as i32, (y + 92) as i32, note, INK3, 15.0);
}
