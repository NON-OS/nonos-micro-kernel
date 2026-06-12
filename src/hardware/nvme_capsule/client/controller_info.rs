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
use super::super::protocol::{encode_request, CONTROLLER_INFO_PAYLOAD_LEN, OP_CONTROLLER_INFO};
use super::read::{u16_at, u32_at, u64_at};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeControllerInfo {
    pub cap: u64,
    pub version: u32,
    pub cc: u32,
    pub csts: u32,
    pub aqa: u32,
    pub intms: u32,
    pub intmc: u32,
    pub cmbloc: u32,
    pub cmbsz: u32,
    pub max_queue_entries: u16,
    pub timeout_units: u8,
    pub doorbell_stride: u8,
    pub min_page_shift: u8,
    pub max_page_shift: u8,
    pub nvm_supported: u8,
    pub ready: u8,
    pub fatal: u8,
}

pub fn controller_info() -> Result<NvmeControllerInfo, DriverNvmeError> {
    let _caller = gate_call()?;
    let request_id = next_request_id();
    let frame = encode_request(OP_CONTROLLER_INFO, 0, request_id, &[]);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    decode(&resp.body)
}

fn decode(body: &[u8]) -> Result<NvmeControllerInfo, DriverNvmeError> {
    if body.len() < CONTROLLER_INFO_PAYLOAD_LEN {
        return Err(DriverNvmeError::ProtocolMismatch);
    }
    Ok(NvmeControllerInfo {
        cap: u64_at(body, 0),
        version: u32_at(body, 8),
        cc: u32_at(body, 12),
        csts: u32_at(body, 16),
        aqa: u32_at(body, 20),
        intms: u32_at(body, 24),
        intmc: u32_at(body, 28),
        cmbloc: u32_at(body, 32),
        cmbsz: u32_at(body, 36),
        max_queue_entries: u16_at(body, 40),
        timeout_units: body[42],
        doorbell_stride: body[43],
        min_page_shift: body[44],
        max_page_shift: body[45],
        nvm_supported: body[46],
        ready: body[47],
        fatal: body[48],
    })
}
