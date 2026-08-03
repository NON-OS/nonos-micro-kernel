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
    if message.len() + 1 > REGULAR_PAYLOAD_SIZE {
        return None;
    }
    let mut out = Vec::with_capacity(REGULAR_PAYLOAD_SIZE);
    out.extend_from_slice(message);
    out.push(0x01);
    out.resize(REGULAR_PAYLOAD_SIZE, 0);
    Some(out)
}
