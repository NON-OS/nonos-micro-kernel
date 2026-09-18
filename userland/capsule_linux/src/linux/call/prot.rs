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


//! `mprotect`, and the rule that makes it necessary.
//!
//! NONOS refuses to map a page writable and executable at once, so a
//! loader cannot ask for the end state up front: it writes the pages,
//! then asks for them to be executable. This is the call that makes the
//! second half possible, and it is the reason the kernel gained a
//! protection primitive rather than the loader gaining an exception.

use nonos_libc::peer::{mk_peer_protect, PEER_PROT_EXEC, PEER_PROT_WRITE};

use crate::linux::abi::errno;
use crate::linux::guest::{page_down, page_up, Guest};

pub const PROT_WRITE: u64 = 2;
pub const PROT_EXEC: u64 = 4;

/// A request for both at once. Linux allows it and this system does not,
/// so it is refused here with the errno Linux uses for a protection the
/// policy forbids, rather than silently granted as one or the other.
pub fn wx_refused(prot: u64) -> bool {
    prot & PROT_WRITE != 0 && prot & PROT_EXEC != 0
}

/// Set the protection of a span already mapped in the guest.
pub fn protect_span(guest: &Guest, addr: u64, span: u64, prot: u64) -> i64 {
    let mut bits = 0;
    if prot & PROT_WRITE != 0 {
        bits |= PEER_PROT_WRITE;
    }
    if prot & PROT_EXEC != 0 {
        bits |= PEER_PROT_EXEC;
    }
    mk_peer_protect(guest.pid, addr, span, bits)
}

pub fn mprotect(guest: &mut Guest, addr: u64, len: u64, prot: u64) -> u64 {
    if len == 0 {
        return errno::ok(0);
    }
    if wx_refused(prot) {
        return errno::fail(errno::EPERM);
    }
    let start = page_down(addr);
    let span = page_up(addr + len) - start;
    if protect_span(guest, start, span, prot) < 0 {
        return errno::fail(errno::EACCES);
    }
    errno::ok(0)
}
