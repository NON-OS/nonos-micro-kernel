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

use alloc::vec::Vec;

use super::entries::Entry;
use super::scroll::ensure_visible;
use super::state::State;
use super::view_sort::sort_view;

// Rebuild the visible list from the full listing: keep entries whose name
// contains the active filter, order them by the current sort mode, then keep
// the cursor and scroll window in range. Called whenever the listing, filter,
// or sort mode changes.
pub fn rebuild_view(state: &mut State) {
    let needle = state.filter.to_ascii_lowercase();
    let mut view: Vec<Entry> = state
        .all
        .iter()
        .filter(|e| needle.is_empty() || e.label.to_ascii_lowercase().contains(needle.as_str()))
        .cloned()
        .collect();
    sort_view(&mut view, state.sort_mode);
    state.entries = view;
    if state.cursor >= state.entries.len() {
        state.cursor = state.entries.len().saturating_sub(1);
    }
    ensure_visible(state);
}
