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

use alloc::vec::Vec;

use super::super::capability::gate_reseed;
use super::super::error::EntropyCapsuleError;
use super::super::protocol::{encode_request, MAX_RESEED_BYTES, OP_RESEED};
use super::seq::next_request_id;
use super::transport::round_trip;

pub fn reseed(seed: &[u8]) -> Result<(), EntropyCapsuleError> {
    gate_reseed()?;
    if seed.is_empty() {
        return Err(EntropyCapsuleError::InvalidArgument);
    }
    if seed.len() > MAX_RESEED_BYTES as usize {
        return Err(EntropyCapsuleError::OversizedRequest);
    }
    let mut body = Vec::with_capacity(4 + seed.len());
    body.extend_from_slice(&(seed.len() as u32).to_le_bytes());
    body.extend_from_slice(seed);
    let request_id = next_request_id();
    let frame = encode_request(OP_RESEED, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 || !resp.body.is_empty() {
        return Err(map_status(resp.status));
    }
    Ok(())
}

fn map_status(status: i32) -> EntropyCapsuleError {
    match status {
        -13 => EntropyCapsuleError::AccessDenied,
        -22 => EntropyCapsuleError::InvalidArgument,
        -90 => EntropyCapsuleError::OversizedRequest,
        _ => EntropyCapsuleError::TransportFailure,
    }
}
