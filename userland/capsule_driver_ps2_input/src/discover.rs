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
use nonos_libc::{mk_device_list, DeviceRecord, BUS_KIND_ACPI};
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
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..count] {
        if r.bus_kind != BUS_KIND_ACPI || r.vendor != vendor || r.device != device {
            continue;
        }
        if require_bar && r.bar_count == 0 {
            continue;
        }
        return Some(Found { device_id: r.device_id, irq_line: r.irq_line });
    }
    None
}