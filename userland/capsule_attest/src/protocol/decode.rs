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
        return Err((empty(), E_BAD_LEN));
    }
    let req = Request {
        op: u16_le(buf, 6),
        flags: u16_le(buf, 8),
        request_id: u32_le(buf, 12),
    };
    if u32_le(buf, 0) != MAGIC {
        return Err((req, E_BAD_MAGIC));
    }
    if u16_le(buf, 4) != VERSION {
        return Err((req, E_BAD_VERSION));
    }
    let payload_len = u32_le(buf, 16) as usize;
    if buf.len() < HDR_LEN + payload_len {
        return Err((req, E_BAD_LEN));
    }
    Ok((req, &buf[HDR_LEN..HDR_LEN + payload_len]))
}

fn empty() -> Request {
    Request { op: 0, flags: 0, request_id: 0 }
}

fn u16_le(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}

fn u32_le(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}
