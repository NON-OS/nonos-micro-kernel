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

use nonos_app_skeleton::clients::vfs::{list_paths, stat_full, usage};
use nonos_app_skeleton::discover::lookup_service;

use super::entries::build_entries;
use super::state::State;
use super::view::rebuild_view;

// Above this many entries the per-file stat pass is skipped so a huge directory
// does not cost one IPC round-trip per file on every refresh.
const META_STAT_LIMIT: usize = 128;

pub fn refresh(state: &mut State) {
    state.preview = None;
    if state.owner_pid == 0 {
        state.owner_pid = lookup_service(b"app.file_manager").map(|peer| peer.pid).unwrap_or(0);
    }
    match list_paths(state.owner_pid, state.prefix.as_bytes()) {
        Ok(paths) => {
            state.all = build_entries(state.prefix.as_str(), &paths);
            fill_meta(state);
            state.usage = usage(state.owner_pid).ok();
            rebuild_view(state);
            state.status = if state.entries.is_empty() { b"empty directory" } else { b"click or Enter to open" };
        }
        Err(_) => {
            if state.all.is_empty() {
                state.cursor = 0;
                state.scroll = 0;
                state.status = b"vfs unavailable";
            } else {
                state.status = b"refresh deferred";
            }
        }
    }
}

// Stat each entry so the listing can show file sizes, directory child counts,
// and modified dates. A failed stat leaves the entry without metadata rather
// than aborting the listing.
fn fill_meta(state: &mut State) {
    if state.all.len() > META_STAT_LIMIT {
        return;
    }
    let pid = state.owner_pid;
    for entry in state.all.iter_mut() {
        if let Ok((size, _is_dir, mtime, writable)) = stat_full(pid, entry.full_path.as_bytes()) {
            entry.size = Some(size);
            entry.mtime = mtime;
            entry.writable = writable;
        }
    }
}
