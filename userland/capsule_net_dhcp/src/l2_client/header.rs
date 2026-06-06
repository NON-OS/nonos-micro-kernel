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

use super::wire::{L2_HDR_LEN, L2_MAGIC, L2_VERSION};

pub fn write_request(out: &mut [u8], op: u16, request_id: u32, payload_len: u32) -> usize {
    if out.len() < L2_HDR_LEN {
        return 0;
    }
    out[0..4].copy_from_slice(&L2_MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&L2_VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
    L2_HDR_LEN + payload_len as usize
}

pub fn parse_response(buf: &[u8]) -> Option<(u16, u16, u32, u32)> {
    if buf.len() < L2_HDR_LEN {
        return None;
    }
    if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != L2_MAGIC {
        return None;
    }
    if u16::from_le_bytes([buf[4], buf[5]]) != L2_VERSION {
        return None;
    }
    let op = u16::from_le_bytes([buf[6], buf[7]]);
    let errno = u16::from_le_bytes([buf[8], buf[9]]);
    let request_id = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]);
    Some((op, errno, request_id, payload_len))
}
