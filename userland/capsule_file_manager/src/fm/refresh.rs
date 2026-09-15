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

use nonos_app_skeleton::clients::vfs::{journal_list, list_paths, usage};
use nonos_libc::mk_getpid;

use super::entries::{build_entries, RESERVED_PREFIX};
use super::refresh_meta::fill_meta;
use super::state::State;
use super::tags_reconcile::reconcile;
use super::view::rebuild_view;

pub fn refresh(state: &mut State) {
    state.preview = None;
    state.dir_info = None;
    state.place_stats.clear();
    if state.owner_pid == 0 {
        // Send our own authoritative pid: the vfs server's anti-impersonation
        // check requires the claimed owner pid to equal the real sender pid, so
        // resolving it from a service lookup (which can miss or lag) makes every
        // request fail as EACCES and the directory read as "vfs unavailable".
        state.owner_pid = mk_getpid();
    }
    refresh_recents(state);
    state.usage = usage(state.owner_pid).ok();
    match list_paths(state.owner_pid, state.prefix.as_bytes()) {
        Ok(paths) => {
            state.all = build_entries(state.prefix.as_str(), &paths);
            reconcile(&mut state.tags, state.prefix.as_str(), &paths);
            fill_meta(state);
            rebuild_view(state);
            state.status = if state.entries.is_empty() {
                b"empty directory"
            } else {
                b"click or Enter to open"
            };
        }
        Err(e) => {
            if state.all.is_empty() {
                state.cursor = 0;
                state.scroll = 0;
                // Surface the concrete failure ("vfs ipc failed" = no reply,
                // "access denied" = caller check, etc.) instead of collapsing
                // every case to a generic string, so a wiring bug is diagnosable
                // straight from the window.
                state.status = e.as_bytes();
            } else {
                state.status = b"refresh deferred";
            }
        }
    }
}

// The journal is the Home and Recents surfaces' only input. It carries raw
// store paths, so the manager's own sidecar blobs are dropped here rather than
// leaking into a surface as if the user had opened them. A journal read that
// fails leaves the last good list in place instead of blanking the surface.
const RECENT_LIMIT: u32 = 64;

fn refresh_recents(state: &mut State) {
    if let Ok(entries) = journal_list(state.owner_pid, RECENT_LIMIT) {
        state.recents =
            entries.into_iter().filter(|(_, p)| !p.starts_with(RESERVED_PREFIX)).collect();
    }
}
