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
use nonos_libc::mk_irq_ack;
use super::driver::Driver;
use super::{claim, dma, irq, registers};
use crate::constants::LEG_CFG_CAPACITY;
use crate::debug;
use crate::discover::find_virtio_blk;
use crate::init::bring_up;
use crate::queue::Queue;
pub fn run() -> Result<Driver, &'static str> {
    debug::marker(b"setup: discover");
    let dev = find_virtio_blk().ok_or("no virtio-blk device")?;
    debug::marker(b"setup: claim");
    let claim_epoch = claim::claim(dev.device_id)?;
    debug::marker(b"setup: registers");
    let register_grant = registers::grant(dev, claim_epoch)?;
    debug::marker(b"setup: irq");
    let irq_grant = irq::bind(dev, claim_epoch, register_grant)?;
    debug::marker(b"setup: dma queue");
    let queue_dma = dma::map_queue(dev.device_id, claim_epoch, register_grant, &irq_grant)?;
    debug::marker(b"setup: dma header");
    let header_dma =
        dma::map_header(dev.device_id, claim_epoch, register_grant, &irq_grant, &queue_dma)?;
    debug::marker(b"setup: dma data");
    let data_dma = dma::map_data(
        dev.device_id,
        claim_epoch,
        register_grant,
        &irq_grant,
        &queue_dma,
        &header_dma,
    )?;
    let regs = register_grant.regs();
    debug::marker(b"setup: bring_up");
    let init = bring_up(regs, queue_dma.device_addr, Queue::max_supported_size())?;
    let queue = Queue::new(
        queue_dma.user_va,
        queue_dma.device_addr,
        init.queue_size,
        header_dma.user_va,
        header_dma.device_addr,
        data_dma.user_va,
        data_dma.device_addr,
    );
    let capacity_sectors = unsafe { regs.r64(LEG_CFG_CAPACITY) };
    if capacity_sectors == 0 {
        debug::marker(b"setup: zero capacity");
        return Err("virtio-blk: zero capacity");
    }
    if mk_irq_ack(irq_grant.grant_id) < 0 {
        debug::marker(b"setup: irq ack failed");
        return Err("virtio-blk: irq ack failed");
    }
    debug::marker(b"setup: done");
    Ok(Driver { irq_grant: irq_grant.grant_id, queue, regs, capacity_sectors })
}
