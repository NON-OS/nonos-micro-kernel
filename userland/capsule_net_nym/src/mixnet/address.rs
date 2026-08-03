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

use crate::sphinx::constants::NODE_ADDRESS_LENGTH;

/// IPv4 marker in the first byte of a routing address.
const IP_V4: u8 = 4;

/// Encode where to reach a mix, the way the network expects it.
///
/// A hop is addressed by its socket address, not by its identity key: version
/// byte, big-endian port, then the IP octets, zero padded to the field width.
/// Putting the identity there instead builds a header that decrypts correctly
/// at every hop and routes nowhere, which no offline test detects.
pub fn routing_address(ip: [u8; 4], port: u16) -> [u8; NODE_ADDRESS_LENGTH] {
    let mut out = [0u8; NODE_ADDRESS_LENGTH];
    out[0] = IP_V4;
    out[1..3].copy_from_slice(&port.to_be_bytes());
    out[3..7].copy_from_slice(&ip);
    out
}
