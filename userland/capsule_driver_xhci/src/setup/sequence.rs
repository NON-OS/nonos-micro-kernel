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
use super::{claim::claim, driver::Driver, irq_bind::irq_bind, mmio_map::mmio_map, require_window::require_window};
use crate::controller::{halt, issue_noop_and_wait, program_command_ring, program_dcbaa, program_event_ring, refuse_unsupported, reset, start, wait_cnr_clear, wait_hc_running, ControllerLayout, Scratchpads};
use crate::debug::marker;
use crate::discover::find_xhci;
use crate::dma::DmaPool;
use crate::error::{XhciError, XhciResult};
use crate::handles::BrokerHandles;
use crate::regs::cap::{caplength, context_size, dboff, max_ports, max_scratchpad, max_slots, rtsoff};
use crate::regs::runtime::{imod_program, interrupter_addr};
use crate::rings::{command::CommandRing, event::EventRing};
use crate::slots::SlotTable;
pub fn run() -> XhciResult<Driver> {
    let dev = find_xhci().ok_or(XhciError::DeviceNotFound)?;
    let claim_epoch = claim(dev.device_id)?;
    let mmio = mmio_map(dev.device_id, claim_epoch, dev.bar0_size)?;
    let irq = irq_bind(dev, claim_epoch, &mmio)?;
    let handles = BrokerHandles::new(dev.device_id, mmio.grant_id, mmio.user_va, irq.grant_id);
    let mmio_base = handles.mmio_user_va();
    refuse_unsupported(mmio_base)?;
    let cap_len = caplength(mmio_base) as u64;
    let op_base = mmio_base + cap_len;
    let runtime_offset = rtsoff(mmio_base);
    let doorbell_offset = dboff(mmio_base);
    require_window(cap_len, mmio.length)?;
    require_window(runtime_offset, mmio.length)?;
    require_window(doorbell_offset, mmio.length)?;
    let runtime_base = mmio_base + runtime_offset;
    let doorbell_base = mmio_base + doorbell_offset;
    let primary_intr_base = interrupter_addr(runtime_base, 0);
    let max_slots_val = max_slots(mmio_base);
    let max_ports_val = max_ports(mmio_base);
    let max_scratchpad_val = max_scratchpad(mmio_base);
    let context_size_val = context_size(mmio_base);
    halt(op_base)?;
    reset(op_base)?;
    marker(b"reset ok");
    wait_cnr_clear(op_base)?;
    marker(b"cnr cleared");
    let dma_pool = DmaPool::new(dev.device_id, claim_epoch);
    let scratchpads = Scratchpads::allocate(&dma_pool, max_scratchpad_val)?;
    marker(b"scratchpads ok");
    let dcbaa = program_dcbaa(&dma_pool, op_base, max_slots_val, scratchpads.array_phys())?;
    marker(b"dcbaa ok");
    let mut command_ring = CommandRing::new(&dma_pool)?;
    program_command_ring(op_base, &command_ring);
    marker(b"cmd ring ok");
    let mut event_ring = EventRing::new(&dma_pool)?;
    imod_program(primary_intr_base, 4000, 0);
    program_event_ring(primary_intr_base, &event_ring);
    marker(b"evt ring ok");
    start(op_base);
    wait_hc_running(op_base)?;
    marker(b"running");
    issue_noop_and_wait(doorbell_base, primary_intr_base, &mut command_ring, &mut event_ring)?;
    marker(b"noop ok");
    let layout = ControllerLayout { op_base, doorbell_base, primary_intr_base, max_slots: max_slots_val, max_ports: max_ports_val, max_scratchpad: max_scratchpad_val, context_size: context_size_val };
    marker(b"endpoint driver.xhci0 ready");
    Ok(Driver { handles, dcbaa, scratchpads, dma_pool, command_ring, event_ring, layout, slots: SlotTable::new() })
}
