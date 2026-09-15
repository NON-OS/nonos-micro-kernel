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

use super::state::State;
use super::store_meta::save_meta;

/// Pin or unpin whatever the cursor is on, which is what populates the
/// sidebar's FAVORITES section. The sidecar is written straight away: a pin the
/// user cannot see survive a restart is not a pin.
pub fn toggle_favorite(state: &mut State) {
    let Some(entry) = state.entries.get(state.cursor) else {
        state.status = b"nothing selected";
        return;
    };
    let path = entry.full_path.clone();
    if state.favorites.list().iter().any(|p| *p == path.as_str()) {
        state.favorites.remove(path.as_str());
        state.status = b"unpinned";
    } else if state.favorites.add(path.as_str()) {
        state.status = b"pinned to favorites";
    } else {
        state.status = b"favorites full";
        return;
    }
    save_meta(state);
}
