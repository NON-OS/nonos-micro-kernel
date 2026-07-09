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

use super::state::State;

// Check or uncheck the entry under the cursor.
pub fn toggle(state: &mut State) {
    let Some(entry) = state.entries.get(state.cursor) else { return };
    let path = entry.full_path.clone();
    match state.selected.iter().position(|p| *p == path) {
        Some(i) => {
            state.selected.remove(i);
        }
        None => state.selected.push(path),
    }
    state.status = if state.selected.is_empty() { b"selection cleared" } else { b"selected" };
}
