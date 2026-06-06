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

use crate::protocol::{E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, MAGIC};

pub const HDR_LEN: usize = 20;

#[derive(Clone, Copy)]
pub struct Request {
    pub op: u16,
    pub request_id: u32,
}

pub fn parse(buf: &[u8]) -> Result<(Request, &[u8]), u16> {
    if buf.len() < HDR_LEN {
        return Err(E_BAD_LEN);
    }
    if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC {
        return Err(E_BAD_MAGIC);
    }
    if u16::from_le_bytes([buf[4], buf[5]]) != 1 {
        return Err(E_BAD_VERSION);
    }
    let op = u16::from_le_bytes([buf[6], buf[7]]);
    let request_id = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as usize;
    let want = HDR_LEN + payload_len;
    if buf.len() < want {
        return Err(E_BAD_LEN);
    }
    Ok((Request { op, request_id }, &buf[HDR_LEN..want]))
}
