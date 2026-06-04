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
use super::ring_doorbell::ring_doorbell;
use super::wait_transfer_completion::wait_transfer_completion;
use crate::error::XhciResult;
use crate::rings::event::EventRing;
use crate::rings::transfer::TransferRing;
use crate::trb::builders::data_stage::data_stage_in;
use crate::trb::builders::data_stage_out::data_stage_out;
use crate::trb::builders::setup_stage_generic::{setup_stage, SetupDir};
use crate::trb::builders::status_stage::status_stage_out;
use crate::trb::builders::status_stage_in::status_stage_in;
const DCI_EP0_BIDIR: u8 = 1;
pub fn issue_control_transfer(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    ep0: &mut TransferRing,
    slot_id: u8,
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    data_phys: u64,
    data_len: u16,
) -> XhciResult<()> {
    let dir = direction(bm_request_type, data_len);
    let cycle = ep0.cycle() != 0;
    ep0.enqueue(setup_stage(bm_request_type, b_request, w_value, w_index, data_len, dir, cycle))?;
    if matches!(dir, SetupDir::DeviceToHost) {
        ep0.enqueue(data_stage_in(data_phys, data_len, ep0.cycle() != 0))?;
    } else if matches!(dir, SetupDir::HostToDevice) {
        ep0.enqueue(data_stage_out(data_phys, data_len, ep0.cycle() != 0))?;
    }
    let status = match dir {
        SetupDir::DeviceToHost => status_stage_out(ep0.cycle() != 0),
        _ => status_stage_in(ep0.cycle() != 0),
    };
    let status_phys = ep0.enqueue(status)?;
    ring_doorbell(doorbell_base, slot_id, DCI_EP0_BIDIR);
    wait_transfer_completion(intr_base, status_phys, evt_ring)
}
fn direction(bm_request_type: u8, data_len: u16) -> SetupDir {
    if data_len == 0 {
        SetupDir::NoData
    } else if bm_request_type & 0x80 != 0 {
        SetupDir::DeviceToHost
    } else {
        SetupDir::HostToDevice
    }
}
