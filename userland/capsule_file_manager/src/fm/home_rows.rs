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

use super::file_color::color;
use super::file_kind::kind_of_name;
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::measure_text::{right_text, truncate_to_width, width_of};
use super::recents_group::parent_of;
use super::sidebar_rows::base_label;
use super::theme::{CY_DIM, INK, INK3, LINE};

/// Home stacks its journal rows tighter than the full-screen surfaces do, so the
/// category and storage rows both keep their band at the default window size.
pub const HOME_ROW_H: u32 = 44;

const ROW_ICON: u32 = 20;
const ICON_GAP: u32 = 14;
const META_GAP: u32 = 12;
const TITLE_PX: f32 = 15.0;
const SUB_PX: f32 = 13.0;

/// One Continue Working row: the entry, the directory it came out of, and how
/// long ago it was touched. The title column is whatever the measured meta
/// column leaves, so a long name is cut on a char boundary instead of running
/// under the timestamp.
pub fn recent_row(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, path: &str, meta: &str) {
    let dir = path.ends_with('/');
    let tint = if dir { CY_DIM } else { color(kind_of_name(path)) };
    let glyph = if dir { Icon::Folder } else { Icon::Doc };
    draw(fb, glyph, x, y + (HOME_ROW_H - ROW_ICON) / 2, ROW_ICON, tint);
    let tx = x + ROW_ICON + ICON_GAP;
    let room = (x + w).saturating_sub(tx + width_of(fb, meta, SUB_PX) + META_GAP);
    let title = base_label(path);
    let cut = truncate_to_width(fb, title.as_str(), TITLE_PX, room);
    let _ = fb.text_ttf(tx as i32, (y + 1) as i32, cut, INK, TITLE_PX);
    let sub = truncate_to_width(fb, parent_of(path), SUB_PX, room);
    let _ = fb.text_ttf(tx as i32, (y + 22) as i32, sub, INK3, SUB_PX);
    right_text(fb, x + w, y + 1, meta, SUB_PX, INK3);
    fb.fill_rect(x, y + HOME_ROW_H - 1, w, 1, LINE);
}
