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

use super::{claim, dma, irq, mmio, primary_surface};
use crate::constants::{GPU_CFG_NUM_SCANOUTS, VG_MAX_SCANOUTS};
use crate::debug;
use crate::device::cmd;
use crate::device::virtqueue::{ControlQueue, QueueLayout};
use crate::discover::find_virtio_gpu;
use crate::driver::Driver;
use crate::init::bring_up;
use crate::regs::Regs;
use crate::state::{FenceCounter, ResourceTable, Scanout, ScanoutTable};

pub fn run() -> Result<Driver, &'static str> {
    let dev = find_virtio_gpu().ok_or("virtio-gpu: device not found")?;
    let claim_epoch = claim::claim(dev.device_id)?;
    let registers = mmio::grant(dev, claim_epoch)?;
    let irq = irq::bind(dev, claim_epoch, registers)?;
    let queue = dma::map_queue(dev.device_id, claim_epoch, registers, &irq)?;
    let regs = registers.regs(dev);
    let init = bring_up(regs, queue.device_addr, dev.pci_device)?;
    if irq.grant_id != 0 {
        let _ = mk_irq_ack(irq.grant_id);
    }
    let layout = QueueLayout::new(init.queue_size, queue.user_va, queue.device_addr)?;
    let control_queue = ControlQueue::new(layout, regs);
    let scanouts = ScanoutTable::new();
    let fences = FenceCounter::new();
    let resources = ResourceTable::new();
    seed_scanouts(&control_queue, &scanouts, &fences)?;
    let primary = scanouts
        .get(0)
        .and_then(|s| s.enabled.then_some(s))
        .map(|s| {
            primary_surface::create(
                dev.device_id,
                claim_epoch,
                &control_queue,
                &fences,
                &resources,
                s,
            )
        })
        .transpose()?
        .flatten();
    emit_claim_trace(regs, init.queue_size);
    Ok(Driver {
        device_id: dev.device_id,
        pci_device: dev.pci_device,
        claim_epoch,
        mmio_grant: registers.grant_id(),
        irq_grant: irq.grant_id,
        queue_grant: queue.grant_id,
        queue_user_va: queue.user_va,
        queue_device_addr: queue.device_addr,
        queue_size: init.queue_size,
        host_features: init.host_features,
        regs,
        control_queue,
        resources,
        scanouts,
        fences,
        primary,
    })
}

const DEFAULT_SCANOUT_WIDTH: u32 = 1024;
const DEFAULT_SCANOUT_HEIGHT: u32 = 768;

fn seed_scanouts(
    q: &ControlQueue,
    table: &ScanoutTable,
    fences: &FenceCounter,
) -> Result<(), &'static str> {
    let info = cmd::get_display_info(q, fences.issue())?;
    let mut recorded = 0usize;
    for i in 0..VG_MAX_SCANOUTS {
        let s = info.scanouts[i];
        if s.enabled == 0 || s.width == 0 || s.height == 0 {
            continue;
        }
        let _ = table.record(
            i as u32,
            Scanout {
                x: s.x,
                y: s.y,
                width: s.width,
                height: s.height,
                current_resource_id: 0,
                enabled: true,
            },
        );
        recorded += 1;
    }
    if recorded == 0 {
        let _ = table.record(
            0,
            Scanout {
                x: 0,
                y: 0,
                width: DEFAULT_SCANOUT_WIDTH,
                height: DEFAULT_SCANOUT_HEIGHT,
                current_resource_id: 0,
                enabled: true,
            },
        );
    }
    Ok(())
}

fn emit_claim_trace(regs: Regs, queue_size: u16) {
    let scanouts = unsafe { regs.r32(GPU_CFG_NUM_SCANOUTS) };
    let mut label = [0u8; 80];
    let prefix = b"device claimed, scanouts=";
    let suffix = b" ctrl_q=";
    let fmt_a = b" ARGB8888 active";
    let mut pos = 0usize;
    pos += copy_to(&mut label[pos..], prefix);
    pos += write_u32(&mut label[pos..], scanouts);
    pos += copy_to(&mut label[pos..], suffix);
    pos += write_u32(&mut label[pos..], queue_size as u32);
    pos += copy_to(&mut label[pos..], fmt_a);
    debug::marker(&label[..pos]);
}

fn copy_to(dst: &mut [u8], src: &[u8]) -> usize {
    let n = if src.len() > dst.len() { dst.len() } else { src.len() };
    dst[..n].copy_from_slice(&src[..n]);
    n
}

fn write_u32(dst: &mut [u8], mut v: u32) -> usize {
    if v == 0 {
        if !dst.is_empty() {
            dst[0] = b'0';
            return 1;
        }
        return 0;
    }
    let mut buf = [0u8; 10];
    let mut n = 0;
    while v > 0 && n < buf.len() {
        buf[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    let out = if n > dst.len() { dst.len() } else { n };
    for i in 0..out {
        dst[i] = buf[n - 1 - i];
    }
    out
}
