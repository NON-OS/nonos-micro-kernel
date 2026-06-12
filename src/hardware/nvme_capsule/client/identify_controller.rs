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
use super::super::error::DriverNvmeError;
use super::super::protocol::{
    encode_request, IDENTIFY_CONTROLLER_PAYLOAD_LEN, OP_IDENTIFY_CONTROLLER,
};
use super::read::{u16_at, u32_at};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeControllerIdentity {
    pub vendor_id: u16,
    pub subsystem_vendor_id: u16,
    pub serial: [u8; 20],
    pub model: [u8; 40],
    pub firmware: [u8; 8],
    pub version: u32,
    pub optional_admin: u16,
    pub namespace_count: u32,
    pub mdts: u8,
    pub sq_entry_size: u8,
    pub cq_entry_size: u8,
    pub optional_nvm: u16,
    pub volatile_write_cache: u8,
}

pub fn identify_controller() -> Result<NvmeControllerIdentity, DriverNvmeError> {
    let _caller = gate_call()?;
    let request_id = next_request_id();
    let frame = encode_request(OP_IDENTIFY_CONTROLLER, 0, request_id, &[]);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    decode(&resp.body)
}

fn decode(body: &[u8]) -> Result<NvmeControllerIdentity, DriverNvmeError> {
    if body.len() < IDENTIFY_CONTROLLER_PAYLOAD_LEN {
        return Err(DriverNvmeError::ProtocolMismatch);
    }
    let mut serial = [0u8; 20];
    let mut model = [0u8; 40];
    let mut firmware = [0u8; 8];
    serial.copy_from_slice(&body[4..24]);
    model.copy_from_slice(&body[24..64]);
    firmware.copy_from_slice(&body[64..72]);
    Ok(NvmeControllerIdentity {
        vendor_id: u16_at(body, 0),
        subsystem_vendor_id: u16_at(body, 2),
        serial,
        model,
        firmware,
        version: u32_at(body, 72),
        optional_admin: u16_at(body, 76),
        namespace_count: u32_at(body, 78),
        mdts: body[82],
        sq_entry_size: body[83],
        cq_entry_size: body[84],
        optional_nvm: u16_at(body, 85),
        volatile_write_cache: body[87],
    })
}
