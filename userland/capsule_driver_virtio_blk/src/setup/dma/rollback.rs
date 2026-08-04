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
use super::super::registers::RegisterGrant;
use nonos_libc::{mk_device_release, mk_dma_unmap, mk_irq_unbind, DmaMapOut, IrqBindOut};
pub fn base(device_id: u64, regs: RegisterGrant, irq: &IrqBindOut) -> Result<(), &'static str> {
    if mk_irq_unbind(irq.grant_id) < 0 {
        return Err("virtio-blk: irq rollback failed");
    }
    if !regs.release() {
        return Err("virtio-blk: register rollback failed");
    }
    if mk_device_release(device_id) < 0 {
        return Err("virtio-blk: device rollback failed");
    }
    Ok(())
}
pub fn queue(
    device_id: u64,
    regs: RegisterGrant,
    irq: &IrqBindOut,
    queue: &DmaMapOut,
) -> Result<(), &'static str> {
    if mk_dma_unmap(queue.grant_id) < 0 {
        return Err("virtio-blk: queue dma rollback failed");
    }
    base(device_id, regs, irq)
}
pub fn header(
    device_id: u64,
    regs: RegisterGrant,
    irq: &IrqBindOut,
    queue_grant: &DmaMapOut,
    header_grant: &DmaMapOut,
) -> Result<(), &'static str> {
    if mk_dma_unmap(header_grant.grant_id) < 0 {
        return Err("virtio-blk: header dma rollback failed");
    }
    queue(device_id, regs, irq, queue_grant)
}
pub fn data(
    device_id: u64,
    regs: RegisterGrant,
    irq: &IrqBindOut,
    queue_grant: &DmaMapOut,
    header_grant: &DmaMapOut,
    data_grant: &DmaMapOut,
) -> Result<(), &'static str> {
    if mk_dma_unmap(data_grant.grant_id) < 0 {
        return Err("virtio-blk: data dma rollback failed");
    }
    header(device_id, regs, irq, queue_grant, header_grant)
}
