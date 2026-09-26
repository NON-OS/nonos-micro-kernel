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

//! The address halves of `sendto` and `recvfrom`.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const AF_INET: u16 = 2;
const SOCKADDR_IN: usize = 16;

/// port then address, both already in network order.
pub(super) fn encode((port, ip): (u16, [u8; 4])) -> [u8; 6] {
    let p = port.to_be_bytes();
    [p[0], p[1], ip[0], ip[1], ip[2], ip[3]]
}

/// Write the source back as a `sockaddr_in` and say how long it is.
pub(super) fn fill(guest: &mut Guest, at: u64, alen: u64, peer: [u8; 6], got: u64) -> u64 {
    let mut sa = [0u8; SOCKADDR_IN];
    sa[0..2].copy_from_slice(&AF_INET.to_le_bytes());
    sa[2..8].copy_from_slice(&peer);
    if guest.write(at, &sa) < sa.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    if alen != 0 && guest.write(alen, &(sa.len() as u32).to_le_bytes()) < 4 {
        return errno::fail(errno::EFAULT);
    }
    got
}
