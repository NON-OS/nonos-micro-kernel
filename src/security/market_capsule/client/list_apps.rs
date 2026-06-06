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
use super::super::error::MarketError;
use super::super::protocol::{encode_request, OP_LIST_APPS};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

pub fn list_apps() -> Result<u32, MarketError> {
    let _caller = gate_call()?;
    let body: [u8; 0] = [];
    let request_id = next_request_id();
    let frame = encode_request(OP_LIST_APPS, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    if resp.body.len() < 4 {
        return Err(MarketError::ProtocolMismatch);
    }
    let count = u32::from_le_bytes([resp.body[0], resp.body[1], resp.body[2], resp.body[3]]);
    Ok(count)
}
