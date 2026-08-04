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

use crate::sphinx::constants::REGULAR_PAYLOAD_SIZE;
use alloc::vec::Vec;

/// Pad a message to the fixed payload width.
///
/// The scheme is a single 0x01 byte after the message, then zeros. Every
/// packet on the wire is the same size whatever it carries, which is the
/// point: length would otherwise identify traffic through the mixnet.
pub fn pad_payload(message: &[u8]) -> Option<Vec<u8>> {
    pad_payload_to(message, REGULAR_PAYLOAD_SIZE)
}

/// Pad to a width other than the usual one.
///
/// Acknowledgements travel in their own narrower packet, so the width a
/// payload is padded to is a property of the packet being built rather than
/// a constant of the protocol.
pub fn pad_payload_to(message: &[u8], width: usize) -> Option<Vec<u8>> {
    if message.len() + 1 > width {
        return None;
    }
    let mut out = Vec::with_capacity(width);
    out.extend_from_slice(message);
    out.push(0x01);
    out.resize(width, 0);
    Some(out)
}
