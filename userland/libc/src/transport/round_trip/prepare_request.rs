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
use crate::transport::envelope::{write_request_v2, RequestV2, HDR_LEN_V2};
use crate::transport::error::TransportError;

use super::types::RoundTrip;

pub fn prepare_request(
    req: &RoundTrip<'_>,
    request_id: u32,
    scratch: &mut [u8],
) -> Result<usize, TransportError> {
    let payload_len = req.payload.len() as u32;
    let total = HDR_LEN_V2 + req.payload.len();
    if scratch.len() < total {
        return Err(TransportError::ResponseTooLarge);
    }
    write_request_v2(
        &mut scratch[..HDR_LEN_V2],
        &RequestV2 {
            magic: req.magic,
            op: req.op,
            flags: req.flags,
            reply_port: req.reply_port,
            request_id,
            payload_len,
        },
    );
    scratch[HDR_LEN_V2..total].copy_from_slice(req.payload);
    Ok(total)
}
