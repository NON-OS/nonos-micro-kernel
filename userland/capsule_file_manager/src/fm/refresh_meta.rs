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

use nonos_app_skeleton::clients::vfs::stat_full;

use super::state::State;

const META_STAT_LIMIT: usize = 128;

pub fn fill_meta(state: &mut State) {
    if state.all.len() > META_STAT_LIMIT {
        return;
    }
    let pid = state.owner_pid;
    for entry in state.all.iter_mut() {
        if entry.is_dir {
            continue;
        }
        if let Ok((size, false, mtime, writable)) = stat_full(pid, entry.full_path.as_bytes()) {
            entry.size = Some(size);
            entry.mtime = mtime;
            entry.writable = writable;
        }
    }
}
