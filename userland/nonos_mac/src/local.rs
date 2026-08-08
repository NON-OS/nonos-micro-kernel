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

/// Length of an Ethernet address.
pub const MAC_LEN: usize = 6;

/// Group bit, bit 0 of the first octet. Set means the frame is addressed to a
/// group rather than to one station, so a source address must never have it.
const GROUP: u8 = 0x01;

/// Locally administered bit, bit 1 of the first octet. Set means the address
/// was not assigned by the IEEE to a manufacturer, which is what makes it
/// honest rather than a forgery of somebody else's OUI.
const LOCAL: u8 = 0x02;

/// Turn six random bytes into an address a station may transmit from.
///
/// Only the first octet is constrained, and only in the two bits that carry
/// meaning: group cleared so the address identifies one station, locally
/// administered set so it does not claim a vendor's registered range. The
/// remaining forty six bits are left as they arrived.
///
/// The caller supplies the randomness. This crate deliberately has no opinion
/// about where entropy comes from, so it stays testable and so a driver cannot
/// accidentally seed it from something predictable that lives in the crate.
#[inline]
pub fn from_random(bytes: [u8; MAC_LEN]) -> [u8; MAC_LEN] {
    let mut mac = bytes;
    apply(&mut mac);
    mac
}

/// The same rule, in place.
///
/// What the drivers call. Returning an array across a crate boundary emits a
/// `memcpy`, and a freestanding capsule has no libc to resolve it against, so
/// the link fails on a symbol that never appears in any source file. Writing
/// through a reference sidesteps that entirely.
#[inline]
pub fn apply(mac: &mut [u8; MAC_LEN]) {
    mac[0] = (mac[0] | LOCAL) & !GROUP;
}

/// Whether an address is one a station may legitimately transmit from: a
/// single station, and not claiming a registered vendor range.
pub fn is_local_unicast(mac: &[u8; MAC_LEN]) -> bool {
    mac[0] & GROUP == 0 && mac[0] & LOCAL != 0
}

/// Whether an address came out of a vendor's registered range, which is what a
/// factory address in an EEPROM or efuse looks like. Broadcasting one of these
/// is what ties a machine to a serial number across every network it joins.
pub fn is_factory_assigned(mac: &[u8; MAC_LEN]) -> bool {
    mac[0] & LOCAL == 0
}
