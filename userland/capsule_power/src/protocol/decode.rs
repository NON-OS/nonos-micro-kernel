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
        return Err((Request { op: 0, flags: 0, request_id: 0 }, E_BAD_LEN));
    }
    let req = Request {
        op: u16::from_le_bytes([buf[6], buf[7]]),
        flags: u16::from_le_bytes([buf[8], buf[9]]),
        request_id: u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]),
    };
    if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC {
        return Err((req, E_BAD_MAGIC));
    }
    if u16::from_le_bytes([buf[4], buf[5]]) != VERSION {
        return Err((req, E_BAD_VERSION));
    }
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as usize;
    if buf.len() < HDR_LEN + payload_len {
        return Err((req, E_BAD_LEN));
    }
    Ok((req, &buf[HDR_LEN..HDR_LEN + payload_len]))
}
