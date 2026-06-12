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

use super::super::capability::gate_call;
use super::super::error::DriverHdaError;
use super::super::protocol::{
    encode_request, MAX_STREAMS, OP_STREAM_LAYOUT, STREAM_ENTRY_BYTES, STREAM_LAYOUT_HEADER_BYTES,
};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HdaStreamInfo {
    pub kind: u8,
    pub local_index: u8,
    pub global_index: u16,
    pub mmio_offset: u32,
}

pub fn stream_layout() -> Result<Vec<HdaStreamInfo>, DriverHdaError> {
    let _caller = gate_call()?;
    let request_id = next_request_id();
    let frame = encode_request(OP_STREAM_LAYOUT, 0, request_id, &[]);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    decode_streams(&resp.body)
}

fn decode_streams(body: &[u8]) -> Result<Vec<HdaStreamInfo>, DriverHdaError> {
    if body.len() < STREAM_LAYOUT_HEADER_BYTES {
        return Err(DriverHdaError::ProtocolMismatch);
    }
    let count = core::cmp::min(read_u32(body, 0) as usize, MAX_STREAMS);
    let need = STREAM_LAYOUT_HEADER_BYTES + count * STREAM_ENTRY_BYTES;
    if body.len() < need {
        return Err(DriverHdaError::ProtocolMismatch);
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let o = STREAM_LAYOUT_HEADER_BYTES + i * STREAM_ENTRY_BYTES;
        out.push(HdaStreamInfo {
            kind: body[o],
            local_index: body[o + 1],
            global_index: u16::from_le_bytes([body[o + 2], body[o + 3]]),
            mmio_offset: read_u32(body, o + 4),
        });
    }
    Ok(out)
}

fn read_u32(body: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([body[off], body[off + 1], body[off + 2], body[off + 3]])
}
