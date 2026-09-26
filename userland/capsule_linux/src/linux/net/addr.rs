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


//! `struct sockaddr_in` out of a guest.

use crate::linux::guest::Guest;

/// family(2) port(2) addr(4), and the rest of the sixteen bytes unused.
const SOCKADDR_IN: usize = 16;
const AF_INET: u16 = 2;

/// Port in network order and address in network order, which is the
/// order net.sockets wants as well, so neither is byte swapped here.
pub fn inet(guest: &Guest, at: u64, len: u64) -> Option<(u16, [u8; 4])> {
    if len < SOCKADDR_IN as u64 {
        return None;
    }
    let raw = guest.read(at, SOCKADDR_IN)?;
    if u16::from_le_bytes([raw[0], raw[1]]) != AF_INET {
        return None;
    }
    let port = u16::from_be_bytes([raw[2], raw[3]]);
    Some((port, [raw[4], raw[5], raw[6], raw[7]]))
}
