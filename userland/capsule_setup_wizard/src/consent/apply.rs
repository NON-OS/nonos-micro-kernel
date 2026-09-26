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

//! Granting and withdrawing, when the review screen commits.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_local_consent_grant, mk_local_consent_revoke};

use super::restore::TOKEN;

/// Apply what the person chose. `keep` is the persistence choice: an amnesic
/// machine keeps nothing, this included.
pub fn apply(allow: bool, was_allowed: bool, keep: bool) {
    match (allow, was_allowed) {
        (true, false) => grant(keep),
        (false, true) => {
            let _ = mk_local_consent_revoke();
            // Zeros prove nothing, so the next boot restores nothing.
            store(&[0u8; 32]);
        }
        _ => {}
    }
}

/// A machine with no key to keep consent with gets it for this boot only,
/// and nothing is written that could be mistaken for more.
fn grant(keep: bool) {
    let Ok(Some(token)) = mk_local_consent_grant() else {
        return;
    };
    if keep {
        store(&token);
    }
}

/*
 * The disk cannot drop a record, only overwrite one of the same length, and
 * only by the file's owner. A token loaded from an earlier boot belongs to
 * nobody, so it is unlinked first and written afresh, which makes it this
 * capsule's to persist over the old record.
 */
fn store(bytes: &[u8; 32]) {
    let pid = mk_getpid();
    let _ = vfs::unlink(pid, TOKEN);
    let _ = vfs::mkdir(pid, b"/nonos");
    let _ = vfs::mkdir(pid, b"/nonos/consent");
    if vfs::write_file(pid, TOKEN, bytes).is_ok() {
        let _ = vfs::persist(pid, TOKEN);
    }
}
