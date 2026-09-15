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

extern crate alloc;

use alloc::vec::Vec;

use super::layout::{content_x, GRID_CELL_H, GRID_CELL_W, GRID_TOP};
use super::state::State;

/// One drawn grid cell: the entry it shows and its top-left corner.
pub struct CellSlot {
    pub index: usize,
    pub x: u32,
    pub y: u32,
}

/// The one layout pass for the icon grid. The first drawn cell is aligned to a
/// row boundary so wrapping stays stable while scrolling, and `paint_grid` and
/// `cell_at` both read the placement from here.
pub fn cell_slots(state: &State) -> Vec<CellSlot> {
    let cols = state.grid_cols.max(1) as usize;
    let start = state.scroll - (state.scroll % cols);
    let left = content_x();
    (start..state.entries.len())
        .take(state.view_rows)
        .enumerate()
        .map(|(vis, index)| CellSlot {
            index,
            x: left + (vis % cols) as u32 * GRID_CELL_W,
            y: GRID_TOP + (vis / cols) as u32 * GRID_CELL_H,
        })
        .collect()
}

/// Which entry the point `(x, y)` lands on, tested against exactly the cells
/// that were drawn.
pub fn cell_at(state: &State, x: u32, y: u32) -> Option<usize> {
    cell_slots(state)
        .into_iter()
        .find(|c| x >= c.x && x < c.x + GRID_CELL_W && y >= c.y && y < c.y + GRID_CELL_H)
        .map(|c| c.index)
}
