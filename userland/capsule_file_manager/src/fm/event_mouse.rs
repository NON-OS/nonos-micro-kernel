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

use super::row_geom::row_at;
use super::state::State;

/// Move the cursor to the list row under `y`, tested against the very slot list
/// `paint_rows` drew from, and report whether a row was hit at all.
pub fn select_row(state: &mut State, y: u32) -> bool {
    match row_at(state, y) {
        Some(index) => {
            state.cursor = index;
            true
        }
        None => false,
    }
}
