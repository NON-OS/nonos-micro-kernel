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

//! Icon-grid view: one `grid_card` per entry on the shared chrome plate. Cells
//! are placed by `grid_geom::cell_slots`, the same list `cell_at` hit-tests
//! against, so a click lands on the card that was drawn.

use nonos_app_skeleton::PaintBuffer;

use super::grid_card::card;
use super::grid_geom::cell_slots;
use super::layout::{GRID_CELL_H, GRID_CELL_W};
use super::selection_is_selected::is_selected;
use super::state::State;

// The plate is inset inside its cell so neighbouring cards never touch.
const INSET_X: u32 = 6;
const INSET_Y: u32 = 3;

pub fn paint_grid(state: &State, fb: &mut PaintBuffer) {
    let pw = GRID_CELL_W - INSET_X * 2;
    let ph = GRID_CELL_H - INSET_Y * 2;
    for cell in cell_slots(state) {
        let entry = &state.entries[cell.index];
        let lit = cell.index == state.cursor || is_selected(state, &entry.full_path);
        card(fb, entry, cell.x + INSET_X, cell.y + INSET_Y, pw, ph, lit);
    }
}
