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

use super::super::capability::gate_call;
use super::super::error::DriverBlkError;
use super::super::protocol::{encode_request, OP_CAPACITY};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

pub fn capacity() -> Result<u64, DriverBlkError> {
    let _caller = gate_call()?;
    let body: [u8; 0] = [];
    let request_id = next_request_id();
    let frame = encode_request(OP_CAPACITY, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    if resp.body.len() < 8 {
        return Err(DriverBlkError::ProtocolMismatch);
    }
    Ok(u64::from_le_bytes([
        resp.body[0],
        resp.body[1],
        resp.body[2],
        resp.body[3],
        resp.body[4],
        resp.body[5],
        resp.body[6],
        resp.body[7],
    ]))
}
