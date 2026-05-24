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
use crate::trb::builders::normal::normal;
pub fn issue_bulk_transfer(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    ring: &mut TransferRing,
    slot_id: u8,
    endpoint_dci: u8,
    buffer_phys: u64,
    length: u32,
) -> XhciResult<()> {
    let cycle = ring.cycle() != 0;
    let trb = normal(buffer_phys, length, cycle, true, false);
    let issued_phys = ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, slot_id, endpoint_dci);
    wait_transfer_completion(intr_base, issued_phys, evt_ring)
}