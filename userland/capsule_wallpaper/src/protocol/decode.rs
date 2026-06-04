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

use super::{
    read_u16, read_u32, Request, E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, HDR_LEN, MAGIC, VERSION,
};

pub fn parse(buf: &[u8]) -> Result<(Request, &[u8]), (i32, Request)> {
    if buf.len() < HDR_LEN {
        return Err((E_BAD_LEN, Request { op: 0, flags: 0, request_id: 0 }));
    }
    let op = read_u16(buf, 6).unwrap_or(0);
    let flags = read_u16(buf, 8).unwrap_or(0);
    let request_id = read_u32(buf, 12).unwrap_or(0);
    let req = Request { op, flags, request_id };
    if read_u32(buf, 0) != Some(MAGIC) {
        return Err((E_BAD_MAGIC, req));
    }
    if read_u16(buf, 4) != Some(VERSION) {
        return Err((E_BAD_VERSION, req));
    }
    let Some(payload_len) = read_u32(buf, 16) else {
        return Err((E_BAD_LEN, req));
    };
    let end = match HDR_LEN.checked_add(payload_len as usize) {
        Some(end) if end == buf.len() => end,
        _ => return Err((E_BAD_LEN, req)),
    };
    Ok((req, &buf[HDR_LEN..end]))
}
