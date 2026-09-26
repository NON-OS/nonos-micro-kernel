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

//! Names the guest asked for, and the addresses it was told.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

/// RFC 6598 shared address space: routable nowhere, and reserved against
/// exactly this kind of use.
const BASE: [u8; 2] = [100, 64];

/// A ceiling, because the table is memory a guest grows by asking.
const MAX_NAMES: usize = 4096;

/// The address for `host`, the same one every time it is asked for.
pub fn address_for(guest: &mut Guest, host: &[u8]) -> Option<[u8; 4]> {
    if let Some((_, addr)) = guest.automap.iter().find(|(h, _)| h == host) {
        return Some(*addr);
    }
    if guest.automap.len() >= MAX_NAMES {
        return None;
    }
    let n = guest.automap.len() as u32 + 1;
    let addr = [BASE[0], BASE[1] + (n >> 16) as u8, (n >> 8) as u8, n as u8];
    guest.automap.push((host.to_vec(), addr));
    Some(addr)
}

/// The name an address stands for, when it is one of ours.
pub fn host_for(guest: &Guest, addr: [u8; 4]) -> Option<Vec<u8>> {
    if addr[0] != BASE[0] || addr[1] & 0xC0 != BASE[1] {
        return None;
    }
    guest.automap.iter().find(|(_, a)| *a == addr).map(|(h, _)| h.clone())
}
