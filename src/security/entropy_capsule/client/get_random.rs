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

use super::super::error::EntropyCapsuleError;
use super::super::protocol::{encode_request, MAX_RANDOM_BYTES, OP_GET_RANDOM};
use super::seq::next_request_id;
use super::transport::round_trip;

// Fill `out` with random bytes from the entropy capsule. Returns the
// count written on success. Authorization is the caller's CAP_CRYPTO at
// the CryptoRandom syscall gate; over-sized requests are rejected
// without round-tripping.
pub fn get_random(out: &mut [u8]) -> Result<usize, EntropyCapsuleError> {
    if out.len() > MAX_RANDOM_BYTES as usize {
        return Err(EntropyCapsuleError::OversizedRequest);
    }
    let length = out.len() as u32;
    let body = length.to_le_bytes();
    let request_id = next_request_id();
    let frame = encode_request(OP_GET_RANDOM, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(map_status(resp.status));
    }
    if resp.body.len() != out.len() {
        return Err(EntropyCapsuleError::SourceFailure);
    }
    out.copy_from_slice(&resp.body);
    Ok(out.len())
}

fn map_status(status: i32) -> EntropyCapsuleError {
    match status {
        -22 => EntropyCapsuleError::InvalidArgument,
        -90 => EntropyCapsuleError::OversizedRequest,
        -5 => EntropyCapsuleError::SourceFailure,
        -13 => EntropyCapsuleError::AccessDenied,
        _ => EntropyCapsuleError::TransportFailure,
    }
}
