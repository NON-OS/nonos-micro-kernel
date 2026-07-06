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

use alloc::string::String;
use alloc::vec::Vec;

use super::selection_is_selected::is_selected;
use super::state::State;

pub fn acting(state: &State) -> Vec<(String, bool)> {
    if !state.selected.is_empty() {
        return state
            .entries
            .iter()
            .filter(|e| is_selected(state, &e.full_path))
            .map(|e| (e.full_path.clone(), e.is_dir))
            .collect();
    }
    match state.entries.get(state.cursor) {
        Some(e) => alloc::vec![(e.full_path.clone(), e.is_dir)],
        None => Vec::new(),
    }
}
