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
mod arm;
mod scan;
use crate::error::XhciResult;
use crate::rings::event::EventRing;
use crate::slots::SlotResources;
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
        arm::arm(doorbell_base, res, length)?;
    }
    scan::scan(intr_base, evt_ring, res, length, max_spins)
}
