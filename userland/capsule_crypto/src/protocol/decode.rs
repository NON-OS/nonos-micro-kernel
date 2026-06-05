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

use super::types::{Request, HDR_LEN, MAGIC, MAX_PAYLOAD_BYTES, VERSION};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Short,
    BadMagic,
    BadVersion,
    BadLength,
}

pub fn decode_request(buf: &[u8]) -> Result<Request<'_>, DecodeError> {
    if buf.len() < HDR_LEN {
        return Err(DecodeError::Short);
    }
    let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    if magic != MAGIC {
        return Err(DecodeError::BadMagic);
    }
    let version = u16::from_le_bytes([buf[4], buf[5]]);
    if version != VERSION {
        return Err(DecodeError::BadVersion);
    }
    let op = u16::from_le_bytes([buf[6], buf[7]]);
    let flags = u16::from_le_bytes([buf[8], buf[9]]);
    let request_id = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]);
    if payload_len > MAX_PAYLOAD_BYTES {
        return Err(DecodeError::BadLength);
    }
    let total = HDR_LEN.saturating_add(payload_len as usize);
    if buf.len() < total {
        return Err(DecodeError::BadLength);
    }
    Ok(Request { op, flags, request_id, payload: &buf[HDR_LEN..total] })
}
