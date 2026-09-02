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
use super::driver::Driver;
use super::{claim, dma, irq, registers};
use crate::constants::LEG_CFG_CAPACITY;
use crate::discover::find_virtio_blk;
use crate::init::bring_up;
use crate::queue::Queue;
use nonos_libc::{mk_debug, mk_irq_ack};

const MSIX_CONFIG_SHIFT: usize = 4;

// The step about to run, printed before it. A step that returns an error is
// reported by the retry loop in main; a step that never returns reports
// nothing there, and the last line printed here is then the one that names it.
fn step(name: &str) {
    let mut line = [0u8; 64];
    let tag = b"[BLK] step ";
    let n = tag.len();
    line[..n].copy_from_slice(tag);
    let m = name.len().min(line.len() - n);
    line[n..n + m].copy_from_slice(&name.as_bytes()[..m]);
    let _ = mk_debug(line.as_ptr(), n + m);
}

pub fn run() -> Result<Driver, &'static str> {
    step("find");
    let dev = find_virtio_blk().ok_or("no virtio-blk device")?;
    step("claim");
    let claim_epoch = claim::claim(dev.device_id)?;
    step("regs");
    let register_grant = registers::grant(dev, claim_epoch)?;
    step("irq-bind");
    let (irq_grant, msix) = irq::bind(dev, claim_epoch, register_grant)?;
    step("dma-queue");
    let queue_dma = dma::map_queue(dev.device_id, claim_epoch, register_grant, &irq_grant)?;
    step("dma-header");
    let header_dma =
        dma::map_header(dev.device_id, claim_epoch, register_grant, &irq_grant, &queue_dma)?;
    step("dma-data");
    let data_dma = dma::map_data(
        dev.device_id,
        claim_epoch,
        register_grant,
        &irq_grant,
        &queue_dma,
        &header_dma,
    )?;
    let regs = register_grant.regs();
    step("bring-up");
    let init = match bring_up(regs, queue_dma.device_addr, Queue::max_supported_size()) {
        Ok(init) => init,
        Err(e) => {
            dma::rollback::data(
                dev.device_id,
                register_grant,
                &irq_grant,
                &queue_dma,
                &header_dma,
                &data_dma,
            )?;
            return Err(e);
        }
    };
    let queue = Queue::new(
        queue_dma.user_va,
        queue_dma.device_addr,
        init.queue_size,
        header_dma.user_va,
        header_dma.device_addr,
        data_dma.user_va,
        data_dma.device_addr,
    );
    let capacity_offset =
        if msix { LEG_CFG_CAPACITY + MSIX_CONFIG_SHIFT } else { LEG_CFG_CAPACITY };
    let capacity_sectors = unsafe { regs.r64(capacity_offset) };
    if capacity_sectors == 0 {
        dma::rollback::data(
            dev.device_id,
            register_grant,
            &irq_grant,
            &queue_dma,
            &header_dma,
            &data_dma,
        )?;
        return Err("virtio-blk: zero capacity");
    }
    if mk_irq_ack(irq_grant.grant_id) < 0 {
        dma::rollback::data(
            dev.device_id,
            register_grant,
            &irq_grant,
            &queue_dma,
            &header_dma,
            &data_dma,
        )?;
        return Err("virtio-blk: irq ack failed");
    }
    step("ready");
    Ok(Driver { irq_grant: irq_grant.grant_id, queue, regs, capacity_sectors })
}
