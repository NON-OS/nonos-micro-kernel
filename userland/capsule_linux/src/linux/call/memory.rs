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

//! `brk` and `munmap`. The addresses are chosen here because a
//! Linux program expects a Linux address space, and the kernel only ever
//! maps the pages it is told to.

use crate::linux::abi::errno;
use crate::linux::guest::{page_up, Guest};

/// `brk(0)` reports the break; any other value moves it and reports where
/// it landed, which is Linux's contract and not an error channel.
pub fn brk(guest: &mut Guest, want: u64) -> u64 {
    if want == 0 || want < crate::linux::guest::BRK_BASE {
        return errno::ok(guest.brk);
    }
    let top = page_up(want);
    if top > guest.brk {
        let len = top - guest.brk;
        if guest.map(guest.brk, len, true, false) < 0 {
            return errno::ok(guest.brk);
        }
    }
    guest.brk = want;
    errno::ok(guest.brk)
}

/// Accepted and remembered as unmapped only in the sense that the guest
/// may map over it again. Returning the pages needs an unmap peer call,
/// which is the next primitive; until then the memory stays the guest's.
pub fn munmap(_guest: &mut Guest, _addr: u64, _len: u64) -> u64 {
    errno::ok(0)
}
