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
use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
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
}
pub fn find_virtio_gpu() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if !is_match(r) || r.irq_pin == 0 || r.irq_line == 0xFF {
            continue;
        }
        if let Some((bar, kind, size)) = first_register_bar(r) {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                register_bar: bar,
                register_kind: kind,
                register_size: size,
                pci_device: r.device,
            });
        }
    }
    None
}
fn is_match(r: &DeviceRecord) -> bool {
    r.vendor == VIRTIO_VENDOR_ID
        && r.bus_kind == BUS_KIND_PCI
        && (r.device == VIRTIO_GPU_TRANSITIONAL || r.device == VIRTIO_GPU_MODERN)
}
fn first_register_bar(r: &DeviceRecord) -> Option<(u8, u8, u64)> {
    for kind in [BAR_KIND_PIO, BAR_KIND_MMIO] {
        for i in 0..r.bars.len() {
            let bar = r.bars[i];
            if bar.kind == kind && bar.size != 0 {
                return Some((i as u8, bar.kind, bar.size));
            }
        }
    }
    None
}