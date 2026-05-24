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
use super::{claim, config, dma_set, irq, mmio, queues};
use crate::constants::VIRTIO_NET_F_STATUS;
use crate::discover::find_virtio_net;
use crate::init::{driver_ok, negotiate};
use crate::regs::Regs;

pub fn run() -> Result<Driver, &'static str> {
    let dev = find_virtio_net().ok_or("no virtio-net device")?;
    let claim_epoch = claim::claim(dev.device_id)?;
    let mmio_grant = mmio::map(dev, claim_epoch)?;
    let irq_grant = irq::bind(dev, claim_epoch, &mmio_grant)?;
    let dma = dma_set::map(dev.device_id, claim_epoch, &mmio_grant, &irq_grant)?;
    let regs = Regs::new(mmio_grant.user_va);
    let negotiated = negotiate(regs)?;
    let status_supported = config::feature_enabled(negotiated, VIRTIO_NET_F_STATUS);
    let mac = config::read_mac(regs, negotiated);
    let (rx, tx) = queues::build(regs, &dma)?;
    rx.prime();
    driver_ok(regs);
    if mk_irq_ack(irq_grant.grant_id) < 0 {
        return Err("virtio-net: irq ack failed");
    }

    Ok(Driver {
        device_id: dev.device_id,
        claim_epoch,
        mmio_grant: mmio_grant.grant_id,
        irq_grant: irq_grant.grant_id,
        rx_queue_grant: dma.rx_queue.grant_id,
        rx_buffer_grant: dma.rx_buffer.grant_id,
        tx_queue_grant: dma.tx_queue.grant_id,
        tx_buffer_grant: dma.tx_buffer.grant_id,
        rx,
        tx,
        regs,
        mac,
        status_supported,
    })
}
