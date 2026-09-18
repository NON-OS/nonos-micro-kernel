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


//! Walking a guest's `struct pollfd` array.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::poll::ready;

/// fd, events, revents.
const POLLFD_LEN: usize = 8;
const POLLNVAL: u16 = 0x020;

pub fn poll(guest: &mut Guest, at: u64, count: u64) -> u64 {
    let mut hits = 0;
    for i in 0..count {
        let entry = at + i * POLLFD_LEN as u64;
        let Some(raw) = guest.read(entry, POLLFD_LEN) else {
            return errno::fail(errno::EFAULT);
        };
        let fd = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) as u64;
        let events = u16::from_le_bytes([raw[4], raw[5]]);
        let revents = ready(guest, fd) & (events | POLLNVAL);
        if revents != 0 {
            hits += 1;
        }
        if guest.write(entry + 6, &revents.to_le_bytes()) < 2 {
            return errno::fail(errno::EFAULT);
        }
    }
    errno::ok(hits)
}
