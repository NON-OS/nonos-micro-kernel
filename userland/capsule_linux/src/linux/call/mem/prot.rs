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

use crate::linux::abi::errno;
use crate::linux::guest::{span_within, Guest, STACK_TOP};

use super::prot_span::protect_span;

pub const PROT_WRITE: u64 = 2;
pub const PROT_EXEC: u64 = 4;

/// A request for both at once.
pub fn wx_refused(prot: u64) -> bool {
    prot & PROT_WRITE != 0 && prot & PROT_EXEC != 0
}

pub fn mprotect(guest: &mut Guest, addr: u64, len: u64, prot: u64) -> u64 {
    if len == 0 {
        return errno::ok(0);
    }
    if wx_refused(prot) {
        return errno::fail(errno::EPERM);
    }
    /*
     * Checked, because `len` is the guest's: `addr + len` wraps and the
     * span computed from the wrapped value comes out enormous.
     */
    let Some((start, span)) = span_within(addr, len, STACK_TOP) else {
        return errno::fail(errno::EINVAL);
    };
    if protect_span(guest, start, span, prot) < 0 {
        return errno::fail(errno::EACCES);
    }
    errno::ok(0)
}
