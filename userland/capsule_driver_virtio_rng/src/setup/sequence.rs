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

//! End-to-end setup sequence: discover -> claim -> MMIO -> IRQ ->
//! queue DMA -> buffer DMA -> virtio init -> first IRQ unmask.
//! Each phase emits its own marker on the way past so the boot
//! harness can localise a failure.

use nonos_libc::mk_irq_ack;

use super::driver::Driver;
use super::{claim, dma, irq, registers};
use crate::constants::ENTROPY_BUF_LEN;
use crate::debug;
use crate::discover::find_virtio_rng;
use crate::init::bring_up;
use crate::queue::Queue;

pub fn run() -> Result<Driver, &'static str> {
    debug::marker(b"setup start");
    let dev = find_virtio_rng().ok_or("no virtio-rng device")?;
    debug::marker(b"discover ok");

    let claim_epoch = claim::claim(dev.device_id)?;
    debug::marker(b"claim ok");

    let register_grant = registers::grant(dev, claim_epoch)?;
    debug::marker(b"mmio ok");

    let irq_grant = irq::bind(dev, claim_epoch, register_grant)?;
    debug::marker(b"irq ok");

    let queue_dma = dma::map_queue(dev.device_id, claim_epoch, register_grant, &irq_grant)?;
    debug::marker(b"qdma ok");
    let buf_dma =
        dma::map_buffer(dev.device_id, claim_epoch, register_grant, &irq_grant, &queue_dma)?;
    debug::marker(b"bdma ok");

    let regs = register_grant.regs();
    let queue_size = bring_up(regs, queue_dma.device_addr, Queue::queue_size())?;
    debug::marker(b"bring ok");
    let queue = Queue::new(
        queue_dma.user_va,
        queue_dma.device_addr,
        buf_dma.user_va,
        buf_dma.device_addr,
        ENTROPY_BUF_LEN as u32,
        queue_size,
    );

    let _ = mk_irq_ack(irq_grant.grant_id);
    debug::marker(b"ack ok");

    Ok(Driver {
        device_id: dev.device_id,
        claim_epoch,
        register_grant,
        irq_grant: irq_grant.grant_id,
        queue_grant: queue_dma.grant_id,
        buf_grant: buf_dma.grant_id,
        queue,
        regs,
    })
}
