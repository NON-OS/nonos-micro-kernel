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

use nonos_app_skeleton::measure_ttf;

use super::state::SortMode;

// The header strip's band, its label em size, the checkbox gutter, and the box
// the per-row menu affordance owns.
pub const HEAD_H: u32 = 30;
pub const HEAD_PX: f32 = 13.0;
pub const CHECK_S: u32 = 18;
pub const CHECK_COL: u32 = 30;
pub const MENU_W: u32 = 26;
const COL_PAD: u32 = 12;
const GAP: u32 = 8;
const CHECK_PAD: u32 = 8;

/// A sortable column: the box it owns, its label, and the sort it selects.
pub struct HeadCol {
    pub x: u32,
    pub w: u32,
    pub mode: SortMode,
    pub label: &'static str,
}

/// The one column layout for the detail list: `list_head` draws its strip from
/// it, `paint_row` places every cell from it, and `list_click` tests against it.
pub struct Cols {
    pub check_x: u32,
    pub tile_x: u32,
    pub tag_end: u32,
    pub menu_x: u32,
    pub cols: [HeadCol; 4],
}

/// A trailing column's width: its header label measured off the free function
/// -- so the hit-test sizes it exactly as the painter did, at the size
/// `MIN_UI_PX` clamps to -- padded, then floored at what its values need.
fn col_w(label: &str, min: u32) -> u32 {
    (measure_ttf(label, HEAD_PX).max(0) as u32 + COL_PAD * 2).max(min)
}

/// Right-to-left from the content edge, leaving the name column what is left
/// between the filetype tile and the first meta cell.
pub fn cols(left: u32, cw: u32) -> Cols {
    let menu_x = (left + cw).saturating_sub(MENU_W);
    let (dw, sw, tw) = (col_w("Modified", 128), col_w("Size", 92), col_w("Type", 84));
    let date_x = menu_x.saturating_sub(GAP + dw);
    let size_x = date_x.saturating_sub(GAP + sw);
    let type_x = size_x.saturating_sub(GAP + tw);
    let tile_x = left + CHECK_COL;
    let tag_end = type_x.saturating_sub(GAP);
    let name_w = tag_end.saturating_sub(tile_x);
    let cols = [
        HeadCol { x: tile_x, w: name_w, mode: SortMode::Name, label: "Name" },
        HeadCol { x: type_x, w: tw, mode: SortMode::Type, label: "Type" },
        HeadCol { x: size_x, w: sw, mode: SortMode::Size, label: "Size" },
        HeadCol { x: date_x, w: dw, mode: SortMode::Date, label: "Modified" },
    ];
    Cols { check_x: left + CHECK_PAD, tile_x, tag_end, menu_x, cols }
}
