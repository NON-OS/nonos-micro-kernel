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

//! iwlwifi host-command header. A command handed to the firmware begins with a
//! 4-byte header `[cmd, group, sequence:le16]` followed by the payload. The
//! sequence ties the command to its transmit queue and ring index so the
//! firmware's response notification can be matched back to it. Pure encoding,
//! checked by `iwlwifi_proofs`.

pub const CMD_HEADER_LEN: usize = 4;

/// The legacy command group (group 0). Newer command sets use non-zero groups.
pub const GROUP_LEGACY: u8 = 0;

/// Encode the 4-byte command header from the command id, the group id, and the
/// little-endian sequence.
pub fn cmd_header(cmd: u8, group: u8, sequence: u16) -> [u8; CMD_HEADER_LEN] {
    let s = sequence.to_le_bytes();
    [cmd, group, s[0], s[1]]
}

/// Compose a sequence value from the transmit queue id (5 bits) and the ring
/// index (low byte), the layout the firmware routes its response by.
pub const fn make_sequence(queue: u8, index: u8) -> u16 {
    ((queue as u16 & 0x1F) << 8) | (index as u16)
}

/// The transmit queue carried by a sequence value.
pub const fn seq_queue(sequence: u16) -> u8 {
    ((sequence >> 8) & 0x1F) as u8
}

/// The ring index carried by a sequence value.
pub const fn seq_index(sequence: u16) -> u8 {
    (sequence & 0xFF) as u8
}
