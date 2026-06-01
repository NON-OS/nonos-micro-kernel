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
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::slots::SlotResources;
use crate::trb::builders::normal::normal;
pub fn arm(doorbell_base: u64, res: &mut SlotResources, length: u16) -> XhciResult<()> {
    let buffer_phys = res.int_buf.as_ref().ok_or(XhciError::ControllerUnsupported)?.phys();
    let ring = res.int_ring.as_mut().ok_or(XhciError::ControllerUnsupported)?;
    let cycle = ring.cycle() != 0;
    let trb = normal(buffer_phys, length as u32, cycle, true, false);
    let issued_phys = ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, res.slot_id, res.int_dci);
    res.int_armed = Some(issued_phys);
    Ok(())
}
