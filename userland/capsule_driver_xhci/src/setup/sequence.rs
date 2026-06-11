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
use super::marker::marker;
use super::{
    assemble::assemble, claim::claim, driver::Driver, irq_bind::irq_bind, layout::read_layout,
    mmio_map::mmio_map,
};
use crate::controller::{
    halt, issue_noop_and_wait, program_command_ring, program_dcbaa, program_event_ring, reset,
    start, wait_cnr_clear, wait_hc_running, Scratchpads,
};
use crate::discover::find_xhci;
use crate::dma::DmaPool;
use crate::error::{XhciError, XhciResult};
use crate::handles::BrokerHandles;
use crate::regs::runtime::imod_program;
use crate::rings::{command::CommandRing, event::EventRing};

pub fn run() -> XhciResult<Driver> {
    let dev = find_xhci().ok_or(XhciError::DeviceNotFound)?;
    let claim_epoch = claim(dev.device_id)?;
    let mmio = mmio_map(dev.device_id, claim_epoch, dev.bar0_size)?;
    let irq = irq_bind(dev, claim_epoch, &mmio)?;
    let handles = BrokerHandles::new(dev.device_id, mmio.grant_id, mmio.user_va, irq.grant_id);
    let layout = read_layout(&handles, mmio.length)?;
    halt(layout.op_base)?;
    reset(layout.op_base)?;
    marker(b"[driver_xhci] reset ok\n");
    wait_cnr_clear(layout.op_base)?;
    marker(b"[driver_xhci] cnr cleared\n");
    let dma_pool = DmaPool::new(dev.device_id, claim_epoch);
    let scratchpads = Scratchpads::allocate(&dma_pool, layout.max_scratchpad)?;
    marker(b"[driver_xhci] scratchpads ok\n");
    let dcbaa =
        program_dcbaa(&dma_pool, layout.op_base, layout.max_slots, scratchpads.array_phys())?;
    marker(b"[driver_xhci] dcbaa ok\n");
    let mut command_ring = CommandRing::new(&dma_pool)?;
    program_command_ring(layout.op_base, &command_ring);
    marker(b"[driver_xhci] cmd ring ok\n");
    let mut event_ring = EventRing::new(&dma_pool)?;
    imod_program(layout.primary_intr_base, 4000, 0);
    program_event_ring(layout.primary_intr_base, &event_ring);
    marker(b"[driver_xhci] evt ring ok\n");
    start(layout.op_base);
    wait_hc_running(layout.op_base)?;
    marker(b"[driver_xhci] running\n");
    issue_noop_and_wait(
        layout.doorbell_base,
        layout.primary_intr_base,
        &mut command_ring,
        &mut event_ring,
    )?;
    marker(b"[driver_xhci] noop ok\n");
    marker(b"[driver_xhci] endpoint driver.xhci0 ready\n");
    Ok(assemble(handles, dcbaa, scratchpads, dma_pool, command_ring, event_ring, layout))
}
