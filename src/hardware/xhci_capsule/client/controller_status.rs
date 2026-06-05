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
use super::super::error::DriverXhciError;
use super::super::protocol::{encode_request, CONTROLLER_STATUS_PAYLOAD_LEN, OP_CONTROLLER_STATUS};
use super::seq::next_request_id;
use super::transport::round_trip;

#[derive(Clone, Copy)]
pub struct ControllerStatus {
    pub max_slots: u8,
    pub max_ports: u8,
    pub max_scratchpad: u32,
    pub scratchpad_pages_alloc: u32,
    pub usbsts: u32,
    pub usbcmd: u32,
    pub iman: u32,
    pub cmd_cycle: u8,
    pub events_drained_total: u64,
    pub dcbaa_phys: u64,
    pub scratchpad_array_phys: u64,
    pub allocated_slots: u32,
}

pub fn controller_status() -> Result<ControllerStatus, DriverXhciError> {
    let _caller = gate_call()?;
    let body: [u8; 0] = [];
    let request_id = next_request_id();
    let frame = encode_request(OP_CONTROLLER_STATUS, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(DriverXhciError::DeviceFailure);
    }
    if resp.body.len() < CONTROLLER_STATUS_PAYLOAD_LEN {
        return Err(DriverXhciError::ShortReply);
    }
    let b = &resp.body;
    Ok(ControllerStatus {
        max_slots: b[0],
        max_ports: b[1],
        max_scratchpad: u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
        scratchpad_pages_alloc: u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        usbsts: u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        usbcmd: u32::from_le_bytes([b[16], b[17], b[18], b[19]]),
        iman: u32::from_le_bytes([b[20], b[21], b[22], b[23]]),
        cmd_cycle: b[24],
        events_drained_total: u64::from_le_bytes([
            b[28], b[29], b[30], b[31], b[32], b[33], b[34], b[35],
        ]),
        dcbaa_phys: u64::from_le_bytes([
            b[36], b[37], b[38], b[39], b[40], b[41], b[42], b[43],
        ]),
        scratchpad_array_phys: u64::from_le_bytes([
            b[44], b[45], b[46], b[47], b[48], b[49], b[50], b[51],
        ]),
        allocated_slots: u32::from_le_bytes([b[52], b[53], b[54], b[55]]),
    })
}
