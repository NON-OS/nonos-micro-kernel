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

use nonos_app_skeleton::clients::vfs::{persist, store_remove};

use super::state::State;
use super::store_meta::{save_meta, FAVORITES_KEY, PREFS_KEY, TAGS_KEY};

// The message `errmsg` renders for EEXIST. The block container writes a TOC
// name once and refuses different bytes under it, so this is the one failure
// that means "committed already, under older bytes" rather than a real fault.
const ALREADY: &str = "already exists";

/// Commit the sidecar blobs to the block store so they outlive the boot.
///
/// `save_meta` alone only reaches the vfs store, which is RAM-resident: the
/// write handler never touches the block layer, so without this the tags,
/// favourites and prefs reset on every boot.
///
/// Deliberately coarse. Each blob that actually changed costs one payload
/// extent that `store_remove` cannot reclaim — it frees the TOC slot, not the
/// bytes — so this belongs on a session boundary, never on a keystroke.
pub fn persist_meta(state: &State) {
    save_meta(state);
    for key in [TAGS_KEY, FAVORITES_KEY, PREFS_KEY] {
        persist_key(state.owner_pid, key);
    }
}

// Persist first and only rewrite on EEXIST. Identical bytes are the appender's
// idempotent retry, so an unchanged blob costs nothing at all; anything else
// would burn an extent on every save whether or not the metadata moved.
//
// Removing first would be the destructive order: a persist that then failed
// would leave nothing on disk where a good older copy used to be.
fn persist_key(owner_pid: u32, key: &[u8]) {
    if persist(owner_pid, key) != Err(ALREADY) {
        return;
    }
    if store_remove(key).is_err() {
        return;
    }
    let _ = persist(owner_pid, key);
}
