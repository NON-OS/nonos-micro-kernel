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

use nonos_app_skeleton::clients::vfs::list_paths;

use super::entries::build_entries;
use super::state::State;

pub fn refresh(state: &mut State) {
    state.preview = None;
    match list_paths(state.owner_pid, state.prefix.as_bytes()) {
        Ok(paths) => {
            state.entries = build_entries(state.prefix.as_str(), &paths);
            state.cursor = state.cursor.min(state.entries.len().saturating_sub(1));
            state.status = if state.entries.is_empty() { b"empty directory" } else { b"click or Enter to open" };
        }
        Err(_) => {
            state.entries.clear();
            state.cursor = 0;
            state.status = b"vfs unavailable";
        }
    }
}
