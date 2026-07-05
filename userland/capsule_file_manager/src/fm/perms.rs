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

use nonos_app_skeleton::clients::vfs::chmod;

use super::refresh::refresh;
use super::selection;
use super::state::State;

const MODE_RW: u16 = 0o644;
const MODE_RO: u16 = 0o444;

// Flip the read-only state of the acting set (selection or cursor). An entry
// currently writable becomes read-only and vice versa.
pub fn toggle_readonly(state: &mut State) {
    let act = selection::acting(state);
    if act.is_empty() {
        state.status = b"nothing selected";
        return;
    }
    let pid = state.owner_pid;
    let mut failed = false;
    for (full, _is_dir) in &act {
        let writable = state.all.iter().find(|e| e.full_path == *full).is_none_or(|e| e.writable);
        let mode = if writable { MODE_RO } else { MODE_RW };
        if chmod(pid, full.trim_end_matches('/').as_bytes(), mode).is_err() {
            failed = true;
        }
    }
    selection::clear(state);
    refresh(state);
    state.status = if failed { b"chmod: some failed" } else { b"permissions changed" };
}
