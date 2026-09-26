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

//! Reprotecting a span, a peer call at a time.

use nonos_libc::peer::{mk_peer_protect, PEER_PROT_EXEC, PEER_PROT_WRITE};

use crate::linux::guest::{Guest, MAX_SPAN};

use super::prot::{PROT_EXEC, PROT_WRITE};

/// Set the protection of a span already mapped in the guest.
pub fn protect_span(guest: &Guest, addr: u64, span: u64, prot: u64) -> i64 {
    let mut bits = 0;
    if prot & PROT_WRITE != 0 {
        bits |= PEER_PROT_WRITE;
    }
    if prot & PROT_EXEC != 0 {
        bits |= PEER_PROT_EXEC;
    }
    let mut done = 0;
    while done < span {
        let take = (span - done).min(MAX_SPAN);
        let rc = mk_peer_protect(guest.pid, addr + done, take, bits);
        if rc < 0 {
            return rc;
        }
        done += take;
    }
    0
}
