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

//! Walk the broker's device table and pick the first virtio-rng
//! that exposes an INTx pin. The capsule refuses to drive a device
//! without a usable INTx line; MSI/MSI-X is not yet supported by
//! the broker so a device with `irq_pin == 0` would have no notify
//! path back to userland.

use nonos_libc::{mk_device_list, DeviceRecord};

use super::constants::{VIRTIO_RNG_MODERN, VIRTIO_RNG_TRANSITIONAL, VIRTIO_VENDOR_ID};

const MAX_DEVICES: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub bar0_size: u64,
}

pub fn find_virtio_rng() -> Option<Found> {
    let mut buf: [DeviceRecord; MAX_DEVICES] = [empty_record(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..count] {
        if r.vendor != VIRTIO_VENDOR_ID {
            continue;
        }
        if r.device != VIRTIO_RNG_TRANSITIONAL && r.device != VIRTIO_RNG_MODERN {
            continue;
        }
        if r.irq_pin == 0 || r.irq_line == 0xFF {
            continue;
        }
        if r.bar_count == 0 {
            continue;
        }
        let bar0 = r.bars[0];
        if bar0.size == 0 {
            continue;
        }
        return Some(Found { device_id: r.device_id, irq_line: r.irq_line, bar0_size: bar0.size });
    }
    None
}

fn empty_record() -> DeviceRecord {
    use nonos_libc::Bar;
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
