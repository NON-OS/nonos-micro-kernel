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

use super::chip::{chip, chip_w, CHIP_H};
use super::layout::ROW_H;
use super::list_check::check_box;
use super::list_cols::{cols, CHECK_S};
use super::list_row_meta::row_meta;
use super::measure_text::{truncate_to_width, width_of};
use super::paint_row_tile::row_tile;
use super::recents_group::parent_of;
use super::selection_is_selected::is_selected;
use super::state::State;
use super::theme::{INK, INK3};

const NAME_PX: f32 = 17.0;
const SUB_PX: f32 = 14.0;
const TAG_GAP: u32 = 16;

/// One detail row's content, drawn inside the tile `paint_rows` already laid
/// down. Every cell comes from `list_cols::cols` -- the layout the header strip
/// and the hit-test read too -- and every string is placed by measurement,
/// never by glyph count.
pub fn row_body(state: &State, fb: &mut PaintBuffer, index: usize, y: u32, left: u32, cw: u32) {
    let entry = &state.entries[index];
    let c = cols(left, cw);
    let on = is_selected(state, &entry.full_path);
    check_box(fb, c.check_x, y + (ROW_H - CHECK_S) / 2, on);
    let name_x = row_tile(fb, entry, y, c.tile_x);
    let room = c.tag_end.saturating_sub(name_x + TAG_GAP);
    let name = truncate_to_width(fb, entry.label.trim_end_matches('/'), NAME_PX, room);
    let name_w = width_of(fb, name, NAME_PX);
    let _ = fb.text_ttf(name_x as i32, (y + 1) as i32, name, INK, NAME_PX);
    let loc = truncate_to_width(fb, parent_of(&entry.full_path), SUB_PX, name_w);
    let _ = fb.text_ttf(name_x as i32, (y + 22) as i32, loc, INK3, SUB_PX);

    let mut tx = name_x + name_w + TAG_GAP;
    for tag in state.tags.tags_for(&entry.full_path) {
        if tx + chip_w(tag) > c.tag_end {
            break;
        }
        tx += chip(fb, tx, y + (ROW_H - CHIP_H) / 2, tag, false);
    }
    row_meta(fb, entry, y, &c);
}
