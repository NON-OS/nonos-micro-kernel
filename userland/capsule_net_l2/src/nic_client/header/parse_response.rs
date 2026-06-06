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

use super::super::wire::{NIC_HDR_LEN, NIC_MAGIC, NIC_VERSION};

pub fn parse_response(buf: &[u8]) -> Option<(u16, u32, u32)> {
    if buf.len() < NIC_HDR_LEN {
        return None;
    }
    let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let version = u16::from_le_bytes([buf[4], buf[5]]);
    if magic != NIC_MAGIC || version != NIC_VERSION {
        return None;
    }
    let op = u16::from_le_bytes([buf[6], buf[7]]);
    let request_id = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]);
    Some((op, request_id, payload_len))
}
