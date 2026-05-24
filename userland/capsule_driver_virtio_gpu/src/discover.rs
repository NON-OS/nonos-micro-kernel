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

use nonos_libc::{mk_device_list, Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO};

use crate::constants::{VIRTIO_GPU_MODERN, VIRTIO_GPU_TRANSITIONAL, VIRTIO_VENDOR_ID};

const MAX_DEVICES: usize = 64;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub register_bar: u8,
    pub register_kind: u8,
    pub register_size: u64,
    pub pci_device: u16,
    // Modern virtio register window from the kernel-parsed PCI caps. When
    // `has_virtio`, `register_bar` is the common-cfg BAR, mapped at
    // `common_off`, and the notify register sits `notify_rel` bytes past it.
    pub has_virtio: bool,
    pub common_off: u32,
    pub notify_rel: u32,
}

pub fn find_virtio_gpu() -> Option<Found> {
    let mut buf = [empty_record(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if !is_match(r) || r.irq_pin == 0 || r.irq_line == 0xFF {
            continue;
        }
        if r.virtio_present != 0 {
            let cb = r.virtio_common_bar as usize;
            let size = if cb < r.bars.len() { r.bars[cb].size } else { 0 };
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                register_bar: r.virtio_common_bar,
                register_kind: BAR_KIND_MMIO,
                register_size: size,
                pci_device: r.device,
                has_virtio: true,
                common_off: r.virtio_common_off,
                notify_rel: r.virtio_notify_off.saturating_sub(r.virtio_common_off),
            });
        }
        if let Some((bar, kind, size)) = first_register_bar(r) {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                register_bar: bar,
                register_kind: kind,
                register_size: size,
                pci_device: r.device,
                has_virtio: false,
                common_off: 0,
                notify_rel: 0,
            });
        }
    }
    None
}

fn is_match(r: &DeviceRecord) -> bool {
    r.vendor == VIRTIO_VENDOR_ID
        && (r.device == VIRTIO_GPU_TRANSITIONAL || r.device == VIRTIO_GPU_MODERN)
}

fn first_register_bar(r: &DeviceRecord) -> Option<(u8, u8, u64)> {
    for i in 0..r.bars.len() {
        let bar = r.bars[i];
        if bar.kind == BAR_KIND_PIO && bar.size != 0 {
            return Some((i as u8, bar.kind, bar.size));
        }
    }
    for i in 0..r.bars.len() {
        let bar = r.bars[i];
        if bar.kind == BAR_KIND_MMIO && bar.size != 0 {
            return Some((i as u8, bar.kind, bar.size));
        }
    }
    None
}

fn empty_record() -> DeviceRecord {
    DeviceRecord {
        device_id: 0,
        bus_kind: 0,
        _pad0: [0; 3],
        class: 0,
        vendor: 0,
        device: 0,
        flags: 0,
        bar_count: 0,
        irq_line: 0xFF,
        irq_pin: 0,
        _pad1: [0; 1],
        irq_source: 0,
        bars: [Bar { base: 0, size: 0, kind: 0, flags: 0, _pad: [0; 6] }; 6],
        virtio_present: 0,
        virtio_common_bar: 0,
        virtio_notify_bar: 0,
        virtio_device_bar: 0,
        virtio_common_off: 0,
        virtio_notify_off: 0,
        virtio_device_off: 0,
        virtio_isr_off: 0,
        virtio_notify_mult: 0,
    }
}
