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

use crate::dhcp::constants::*;
use crate::dhcp::message::Message;

pub(super) fn parse_fixed(bytes: &[u8]) -> Message {
    let mut ciaddr = [0u8; 4];
    let mut yiaddr = [0u8; 4];
    let mut chaddr = [0u8; 16];
    ciaddr.copy_from_slice(&bytes[FIELD_CIADDR..FIELD_CIADDR + 4]);
    yiaddr.copy_from_slice(&bytes[FIELD_YIADDR..FIELD_YIADDR + 4]);
    chaddr.copy_from_slice(&bytes[FIELD_CHADDR..FIELD_CHADDR + 16]);
    Message {
        op: bytes[FIELD_OP],
        xid: u32::from_be_bytes([
            bytes[FIELD_XID],
            bytes[FIELD_XID + 1],
            bytes[FIELD_XID + 2],
            bytes[FIELD_XID + 3],
        ]),
        flags: u16::from_be_bytes([bytes[FIELD_FLAGS], bytes[FIELD_FLAGS + 1]]),
        ciaddr,
        yiaddr,
        chaddr,
        ..Default::default()
    }
}
