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
use crate::transport::envelope::{HDR_LEN_V2, VERSION_V2};
use crate::transport::error::TransportError;
use crate::transport::wire::{le_u16, le_u32};

use super::types::{Response, RoundTrip};

pub fn parse_response<'a>(
    req: &RoundTrip<'_>,
    out_buf: &'a [u8],
    n: usize,
    request_id: u32,
) -> Result<Response<'a>, TransportError> {
    if n < HDR_LEN_V2 {
        return Err(TransportError::ResponseTooShort);
    }
    let resp_magic = le_u32(out_buf, 0).ok_or(TransportError::ResponseTooShort)?;
    if resp_magic != req.magic {
        return Err(TransportError::MagicMismatch);
    }
    let version = le_u16(out_buf, 4).ok_or(TransportError::ResponseTooShort)?;
    if version != VERSION_V2 {
        return Err(TransportError::VersionMismatch);
    }
    let op = le_u16(out_buf, 6).ok_or(TransportError::ResponseTooShort)?;
    let errno = le_u16(out_buf, 8).ok_or(TransportError::ResponseTooShort)?;
    let rid = le_u32(out_buf, 16).ok_or(TransportError::ResponseTooShort)?;
    let plen = le_u32(out_buf, 20).ok_or(TransportError::ResponseTooShort)? as usize;
    if rid != request_id {
        return Err(TransportError::RequestIdMismatch);
    }
    let payload_end = HDR_LEN_V2.checked_add(plen).ok_or(TransportError::ResponseTooLarge)?;
    if payload_end > n {
        return Err(TransportError::ResponseTooShort);
    }
    Ok(Response { op, errno, request_id: rid, payload: &out_buf[HDR_LEN_V2..payload_end] })
}
