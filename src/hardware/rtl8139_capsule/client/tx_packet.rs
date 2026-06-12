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
use super::super::error::DriverRtl8139Error;
use super::super::protocol::{
    encode_request, MAX_TX_PAYLOAD_BYTES, MIN_ETHERNET_FRAME, OP_TX_PACKET,
};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

pub fn tx_packet(frame: &[u8]) -> Result<(), DriverRtl8139Error> {
    let _caller = gate_call()?;
    if frame.len() < MIN_ETHERNET_FRAME {
        return Err(DriverRtl8139Error::InvalidArgument);
    }
    if frame.len() as u32 > MAX_TX_PAYLOAD_BYTES {
        return Err(DriverRtl8139Error::OversizedRequest);
    }
    let request_id = next_request_id();
    let frame_req = encode_request(OP_TX_PACKET, 0, request_id, frame);
    let resp = round_trip(request_id, frame_req)?;
    if resp.status == 0 {
        Ok(())
    } else {
        Err(lift(resp.status))
    }
}
