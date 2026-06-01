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
use super::IntrPoll;
use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::error::XhciResult;
use crate::regs::runtime::erdp_program;
use crate::rings::event::EventRing;
use crate::slots::SlotResources;
use crate::trb::Trb;
pub fn scan(
    intr_base: u64,
    evt_ring: &mut EventRing,
    res: &mut SlotResources,
    length: u16,
    _max_spins: u32,
) -> XhciResult<IntrPoll> {
    if !evt_ring.has_event() {
        return Ok(IntrPoll::Pending);
    }
    let event = evt_ring.current_trb();
    if !matches_armed(&event, res.int_armed.unwrap_or(0)) {
        return Ok(IntrPoll::Pending);
    }
    evt_ring.advance();
    erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
    res.int_armed = None;
    Ok(IntrPoll::Complete(transferred(&event, length)))
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
