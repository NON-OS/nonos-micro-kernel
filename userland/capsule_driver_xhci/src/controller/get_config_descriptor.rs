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
use crate::dma::DmaRegion;
use crate::error::XhciResult;
use crate::rings::event::EventRing;
use crate::rings::transfer::TransferRing;
use crate::trb::builders::data_stage::data_stage_in;
use crate::trb::builders::setup_stage::setup_stage_get_descriptor_typed;
use crate::trb::builders::status_stage::status_stage_out;
pub const CONFIG_DESCRIPTOR_TYPE: u8 = 2;
pub const CONFIG_DESCRIPTOR_MAX: u16 = 512;
pub fn get_config_descriptor(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    slot_id: u8,
    ep0: &mut TransferRing,
    out: &DmaRegion,
    length: u16,
) -> XhciResult<usize> {
    let len = core::cmp::min(length, CONFIG_DESCRIPTOR_MAX);
    let setup = setup_stage_get_descriptor_typed(CONFIG_DESCRIPTOR_TYPE, 0, len, ep0.cycle() != 0);
    ep0.enqueue(setup)?;
    ep0.enqueue(data_stage_in(out.phys(), len, ep0.cycle() != 0))?;
    let status_phys = ep0.enqueue(status_stage_out(ep0.cycle() != 0))?;
    ring_doorbell(doorbell_base, slot_id, 1);
    wait_transfer_completion(intr_base, status_phys, evt_ring)?;
    Ok(len as usize)
}
