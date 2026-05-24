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

use crate::constants::pci::{REALTEK_VENDOR_ID, RTL8139_DEVICE_ID};

const MAX_DEVICES: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub pio_bar_index: u8,
    pub command_bits: u16,
}

pub fn find_rtl8139() -> Option<Found> {
    let mut buf: [DeviceRecord; MAX_DEVICES] = [empty_record(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if r.vendor != REALTEK_VENDOR_ID || r.device != RTL8139_DEVICE_ID {
            continue;
        }
        if r.irq_pin == 0 || r.irq_line == 0xFF {
            continue;
        }
        if let Some(pio_bar_index) = first_pio_bar(r) {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                pio_bar_index,
                command_bits: command_bits(r),
            });
        }
    }
    None
}

fn command_bits(r: &DeviceRecord) -> u16 {
    let mut bits = 0u16;
    for i in 0..core::cmp::min(r.bar_count as usize, r.bars.len()) {
        if r.bars[i].kind == BAR_KIND_PIO {
            bits |= 1;
        }
        if r.bars[i].kind == BAR_KIND_MMIO {
            bits |= 2;
        }
    }
    bits
}

fn first_pio_bar(r: &DeviceRecord) -> Option<u8> {
    for i in 0..core::cmp::min(r.bar_count as usize, r.bars.len()) {
        if r.bars[i].kind == BAR_KIND_PIO && r.bars[i].size >= 0x60 {
            return Some(i as u8);
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
