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

//! Map a click in the icon grid to the cell that was drawn there, using the
//! same column count and starting cell paint used this frame.

use nonos_app_skeleton::{InputEvent, InputKind};

use super::layout::{CONTENT_X, GRID_CELL_H, GRID_CELL_W, GRID_PAD_X, GRID_TOP};
use super::state::{Mode, State};

pub fn grid_select(state: &mut State, event: InputEvent) {
    if event.kind != InputKind::ButtonDown || !matches!(state.mode, Mode::Browse) {
        return;
    }
    if event.x < 0 || event.y < 0 {
        return;
    }
    let (x, y) = (event.x as u32, event.y as u32);
    let left = CONTENT_X + GRID_PAD_X / 2;
    if x < left || y < GRID_TOP {
        return;
    }
    let cols = state.grid_cols.max(1);
    let col = (x - left) / GRID_CELL_W;
    if col >= cols {
        return;
    }
    let row = (y - GRID_TOP) / GRID_CELL_H;
    let start = state.scroll - (state.scroll % cols as usize);
    let idx = start + (row * cols + col) as usize;
    if idx < state.entries.len() {
        state.cursor = idx;
    }
}
