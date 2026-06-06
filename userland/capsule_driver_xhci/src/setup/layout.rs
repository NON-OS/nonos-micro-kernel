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
use super::require_window::require_window;
use crate::controller::{refuse_unsupported, ControllerLayout};
use crate::error::XhciResult;
use crate::handles::BrokerHandles;
use crate::regs::cap::{
    caplength, context_size, dboff, max_ports, max_scratchpad, max_slots, rtsoff,
};
use crate::regs::runtime::interrupter_addr;

pub fn read_layout(handles: &BrokerHandles, mmio_len: u64) -> XhciResult<ControllerLayout> {
    let mmio_base = handles.mmio_user_va();
    refuse_unsupported(mmio_base)?;
    let cap_len = caplength(mmio_base) as u64;
    let runtime_offset = rtsoff(mmio_base);
    let doorbell_offset = dboff(mmio_base);
    require_window(cap_len, mmio_len)?;
    require_window(runtime_offset, mmio_len)?;
    require_window(doorbell_offset, mmio_len)?;
    let runtime_base = mmio_base + runtime_offset;
    Ok(ControllerLayout {
        op_base: mmio_base + cap_len,
        doorbell_base: mmio_base + doorbell_offset,
        primary_intr_base: interrupter_addr(runtime_base, 0),
        max_slots: max_slots(mmio_base),
        max_ports: max_ports(mmio_base),
        max_scratchpad: max_scratchpad(mmio_base),
        context_size: context_size(mmio_base),
    })
}
