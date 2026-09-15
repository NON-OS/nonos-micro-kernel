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

use nonos_app_skeleton::clients::vfs::{read_file, write_file};
use nonos_libc::mk_getpid;

use super::favorites::Favorites;
use super::prefs::Prefs;
use super::state::State;
use super::tags::TagMap;

// Reserved sidecar namespace. The leading dot keeps the manager's own
// key/value blobs out of the tree the user browses.
pub(super) const TAGS_KEY: &[u8] = b"/.files/tags";
pub(super) const FAVORITES_KEY: &[u8] = b"/.files/favorites";
pub(super) const PREFS_KEY: &[u8] = b"/.files/prefs";

// Every blob is bounded by its own type (tags cap assignments, favorites cap
// entries), so this is only the ceiling that stops a corrupt file from being
// pulled in whole.
const BLOB_MAX: u32 = 256 * 1024;

/// Restore the persisted sidecar state. Runs before the first refresh, so the
/// pid is resolved the same authoritative way rather than assuming
/// `owner_pid` is already filled in. A key that does not exist yet is the
/// normal first-run case, not an error: each type falls back to its default.
pub fn load_meta(state: &mut State) {
    if state.owner_pid == 0 {
        state.owner_pid = mk_getpid();
    }
    let pid = state.owner_pid;
    if let Ok(buf) = read_file(pid, TAGS_KEY, BLOB_MAX) {
        state.tags = TagMap::from_blob(&buf);
    }
    if let Ok(buf) = read_file(pid, FAVORITES_KEY, BLOB_MAX) {
        state.favorites = Favorites::from_blob(&buf);
    }
    if let Ok(buf) = read_file(pid, PREFS_KEY, BLOB_MAX) {
        state.prefs = Prefs::from_blob(&buf);
    }
    state.view = state.prefs.view;
    state.sort_mode = state.prefs.sort;
}

/// Write the sidecar state back. Best-effort by design: a failed write leaves
/// the in-memory state authoritative for the session rather than taking down
/// the action that produced it.
pub fn save_meta(state: &State) {
    let pid = state.owner_pid;
    let _ = write_file(pid, TAGS_KEY, &state.tags.to_blob());
    let _ = write_file(pid, FAVORITES_KEY, &state.favorites.to_blob());
    let _ = write_file(pid, PREFS_KEY, &state.prefs.to_blob());
}
