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

//! Walk the broker device table for the synthetic PS/2 keyboard
//! record. The kernel registers it once at boot
//! (`broker::register_legacy_platform_devices`); the capsule
//! finds it here by PNP vendor / device.

use nonos_libc::{mk_device_list, Bar, DeviceRecord};

use super::constants::{
    PNP_DEVICE_PS2_AUX, PNP_DEVICE_PS2_KBD, PNP_VENDOR_PS2_AUX, PNP_VENDOR_PS2_KBD,
};

const MAX_DEVICES: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
}

pub fn find_ps2_kbd() -> Option<Found> {
    find_platform(PNP_VENDOR_PS2_KBD, PNP_DEVICE_PS2_KBD, true)
}

pub fn find_ps2_aux() -> Option<Found> {
    find_platform(PNP_VENDOR_PS2_AUX, PNP_DEVICE_PS2_AUX, false)
}

fn find_platform(vendor: u16, device: u16, require_bar: bool) -> Option<Found> {
    let mut buf: [DeviceRecord; MAX_DEVICES] = [empty_record(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..count] {
        if r.vendor != vendor || r.device != device {
            continue;
        }
        if require_bar && r.bar_count == 0 {
            continue;
        }
        return Some(Found { device_id: r.device_id, irq_line: r.irq_line });
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
