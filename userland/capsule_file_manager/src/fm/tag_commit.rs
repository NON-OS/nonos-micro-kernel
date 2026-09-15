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

/// Toggle `name` on the cursor path and write the sidecar back. Typing a tag
/// that is already there removes it, which is the only way to untag from the
/// keyboard, and is why this reports which way it went.
///
/// The tag axis is sidecar metadata, so nothing here touches the filesystem and
/// nothing is pushed onto the undo stack, whose ops are all vfs calls.
pub fn tag_commit(state: &mut State, name: &str) {
    let Some(entry) = state.entries.get(state.cursor) else {
        state.status = b"no selection";
        return;
    };
    let path = entry.full_path.clone();
    state.status = if state.tags.add(path.as_str(), name) {
        b"tagged"
    } else {
        state.tags.remove(path.as_str(), name);
        b"untagged"
    };
    save_meta(state);
}
