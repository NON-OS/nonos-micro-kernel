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

//! Map a click in the icon grid to the cell that was drawn there.

use super::grid_geom::cell_at;
use super::state::State;

/// Move the cursor to the grid cell under `(x, y)`, tested against the very slot
/// list `paint_grid` drew from, and report whether a cell was hit at all. The
/// arithmetic that used to live here assumed its own left edge and knew nothing
/// of the info panel's reserved strip, so it sat a whole padding step out of
/// register with the cells on screen.
pub fn grid_select(state: &mut State, x: u32, y: u32) -> bool {
    match cell_at(state, x, y) {
        Some(index) => {
            state.cursor = index;
            true
        }
        None => false,
    }
}
