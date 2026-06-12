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
use super::super::error::DriverPs2Error;
use super::super::protocol::{encode_request, OP_POLL_EVENTS};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;
use super::types::KeyEvent;

const EVENT_WIRE_LEN: usize = 3;

/// Drain whatever the capsule has buffered since the last poll.
/// Each reply carries up to MAX_POLL_EVENTS (256) events; if the
/// keyboard rate ever exceeds that the capsule drops the oldest
/// (visible via `get_state().events_dropped`).
pub fn poll_events() -> Result<Vec<KeyEvent>, DriverPs2Error> {
    let _caller = gate_call()?;
    let body: [u8; 0] = [];
    let request_id = next_request_id();
    let frame = encode_request(OP_POLL_EVENTS, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    if resp.body.len() < 4 {
        return Err(DriverPs2Error::ProtocolMismatch);
    }
    let count =
        u32::from_le_bytes([resp.body[0], resp.body[1], resp.body[2], resp.body[3]]) as usize;
    let needed = 4 + count * EVENT_WIRE_LEN;
    if resp.body.len() < needed {
        return Err(DriverPs2Error::ProtocolMismatch);
    }
    let mut out = Vec::with_capacity(count);
    let mut off = 4;
    for _ in 0..count {
        out.push(KeyEvent { scancode: resp.body[off], flags: resp.body[off + 1] });
        off += EVENT_WIRE_LEN;
    }
    Ok(out)
}
