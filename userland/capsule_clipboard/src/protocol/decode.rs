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

use super::{Request, E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, HDR_LEN, MAGIC, VERSION};

pub fn parse(buf: &[u8]) -> Result<(Request, &[u8]), (Request, i32)> {
    if buf.len() < HDR_LEN {
        return Err((empty_req(), E_BAD_LEN));
    }
    let op = u16_le(buf, 6);
    let flags = u16_le(buf, 8);
    let request_id = u32_le(buf, 12);
    let req = Request { op, flags, request_id };
    let magic = u32_le(buf, 0);
    if magic != MAGIC {
        return Err((req, E_BAD_MAGIC));
    }
    let version = u16_le(buf, 4);
    if version != VERSION {
        return Err((req, E_BAD_VERSION));
    }
    let payload_len = u32_le(buf, 16) as usize;
    if buf.len() < HDR_LEN + payload_len {
        return Err((req, E_BAD_LEN));
    }
    Ok((req, &buf[HDR_LEN..HDR_LEN + payload_len]))
}

fn empty_req() -> Request {
    Request { op: 0, flags: 0, request_id: 0 }
}

fn u16_le(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([buf[offset], buf[offset + 1]])
}

fn u32_le(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]])
}
