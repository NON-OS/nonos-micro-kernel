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
use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::EventRing;
use crate::slots::SlotResources;
use crate::trb::builders::normal::normal;
use crate::trb::Trb;
pub enum IntrPoll {
    Complete(u16),
    Pending,
}
pub fn poll_interrupt_in(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    res: &mut SlotResources,
    length: u16,
    max_spins: u32,
) -> XhciResult<IntrPoll> {
    if res.int_armed.is_none() {
        arm(doorbell_base, res, length)?;
    }
    scan(intr_base, evt_ring, res, length, max_spins)
}
fn arm(doorbell_base: u64, res: &mut SlotResources, length: u16) -> XhciResult<()> {
    let buffer_phys = res.int_buf.as_ref().ok_or(XhciError::ControllerUnsupported)?.phys();
    let ring = res.int_ring.as_mut().ok_or(XhciError::ControllerUnsupported)?;
    let cycle = ring.cycle() != 0;
    let trb = normal(buffer_phys, length as u32, cycle, true, false);
    let issued_phys = ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, res.slot_id, res.int_dci);
    res.int_armed = Some(issued_phys);
    Ok(())
}
fn scan(
    intr_base: u64,
    evt_ring: &mut EventRing,
    res: &mut SlotResources,
    length: u16,
    max_spins: u32,
) -> XhciResult<IntrPoll> {
    let issued = res.int_armed.unwrap_or(0);
    for _ in 0..max_spins {
        if !evt_ring.has_event() {
            core::hint::spin_loop();
            continue;
        }
        let event = evt_ring.current_trb();
        evt_ring.advance();
        erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
        if !matches_armed(&event, issued) {
            continue;
        }
        res.int_armed = None;
        return Ok(IntrPoll::Complete(transferred(&event, length)));
    }
    Ok(IntrPoll::Pending)
}
fn matches_armed(event: &Trb, issued_phys: u64) -> bool {
    event.get_type() == TRB_TYPE_TRANSFER_EVENT && event.get_pointer() & !0xF == issued_phys & !0xF
}
fn transferred(event: &Trb, length: u16) -> u16 {
    let residual = (event.d2 & 0x00FF_FFFF) as u32;
    let req = length as u32;
    if residual <= req {
        (req - residual) as u16
    } else {
        length
    }
}
