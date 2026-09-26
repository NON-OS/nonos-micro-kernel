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

/// Apply what the person chose. Withdrawing deletes the token too, so the
/// next boot does not quietly restore what was just taken back. `keep` is
/// the persistence choice: an amnesic machine keeps nothing, this included.
pub fn apply(allow: bool, was_allowed: bool, keep: bool) {
    match (allow, was_allowed) {
        (true, false) => grant(keep),
        (false, true) => {
            let _ = mk_local_consent_revoke();
            let _ = vfs::store_remove(TOKEN);
            let _ = vfs::unlink(mk_getpid(), TOKEN);
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
    if !keep {
        return;
    }
    let pid = mk_getpid();
    let _ = vfs::mkdir(pid, b"/nonos");
    let _ = vfs::mkdir(pid, b"/nonos/consent");
    if vfs::write_file(pid, TOKEN, &token).is_ok() {
        let _ = vfs::persist(pid, TOKEN);
    }
}
