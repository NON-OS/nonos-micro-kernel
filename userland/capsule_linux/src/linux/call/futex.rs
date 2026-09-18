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


//! `futex`, entirely in this capsule.
//!
//! A waiter is a guest thread already parked inside its trap, so the wait
//! is the absence of a reply and the wake is the reply. The kernel needs
//! to know nothing about it.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

const FUTEX_WAIT: u64 = 0;
const FUTEX_WAKE: u64 = 1;
const OP_MASK: u64 = 0x7F;

pub fn futex(guest: &mut Guest, tid: u32, uaddr: u64, op: u64, val: u64) -> Answer {
    match op & OP_MASK {
        FUTEX_WAIT => wait(guest, tid, uaddr, val),
        FUTEX_WAKE => Answer::value(errno::ok(guest.wake(uaddr, val))),
        _ => Answer::value(errno::fail(errno::ENOSYS)),
    }
}

fn wait(guest: &mut Guest, tid: u32, uaddr: u64, val: u64) -> Answer {
    let Some(bytes) = guest.read(uaddr, 4) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    let seen = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    // The word changed between the caller's own check and this one, so
    // the condition it was going to sleep on is already false.
    if seen as u64 != val {
        return Answer::value(errno::fail(errno::EAGAIN));
    }
    guest.waits.push((tid, uaddr));
    Answer::Park
}
