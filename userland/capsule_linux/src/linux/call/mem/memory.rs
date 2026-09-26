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

//! `brk` and `munmap`.

use crate::linux::abi::errno;
use crate::linux::guest::{page_up, Guest, BRK_BASE, BRK_LIMIT};

/// `brk(0)` reports the break; any other value moves it and reports where
/// it landed, which is Linux's contract and not an error channel.
pub fn brk(guest: &mut Guest, want: u64) -> u64 {
    /*
     * A break outside the heap area is reported as no move, which is what a
     * Linux program reads as the request being refused.
     */
    if want == 0 || want < BRK_BASE || want > BRK_LIMIT {
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

/// The pages go back to the kernel and leave the guest's region list.
pub fn munmap(guest: &mut Guest, addr: u64, len: u64) -> u64 {
    if len == 0 {
        return errno::fail(errno::EINVAL);
    }
    match guest.unmap(addr, len) {
        rc if rc < 0 => errno::fail(errno::EINVAL),
        _ => errno::ok(0),
    }
}
