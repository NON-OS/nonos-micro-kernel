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

use nonos_app_skeleton::clients::vfs::dirstat;

use super::screen::Screen;
use super::state::State;

/// Bring `state.dir_info` in line with what the cursor is on, at most one vfs
/// round-trip per distinct directory. Runs before paint, never inside it: the
/// panel used to walk the store on every frame, which is a blocking IPC call on
/// a path that has to hold a frame budget.
pub fn sync_info(state: &mut State) {
    if state.screen != Screen::Browse {
        return;
    }
    let target = match state.entries.get(state.cursor) {
        Some(entry) if entry.is_dir => entry.full_path.clone(),
        _ => {
            state.dir_info = None;
            return;
        }
    };
    if state.dir_info.as_ref().is_some_and(|(path, _)| *path == target) {
        return;
    }
    let stat = dirstat(state.owner_pid, target.as_bytes()).ok();
    state.dir_info = Some((target, stat));
}

/// The cached walk for `path`, or `None` when nothing is cached for it yet.
pub fn cached_dir(state: &State, path: &str) -> Option<(u32, u32, u64, bool)> {
    state.dir_info.as_ref().filter(|(key, _)| key == path).and_then(|(_, stat)| *stat)
}
