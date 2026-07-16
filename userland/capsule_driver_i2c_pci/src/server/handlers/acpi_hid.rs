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

use crate::driver::Driver;
use crate::protocol::{Request, E_OK};
use crate::server::respond;

const MAX_DEVICES: usize = 128;
// Mirror of the kernel broker's I2C_HID class id.
const CLASS_I2C_HID: u32 = 0x0041;

/// Answer with the touchpad the kernel registered from ACPI: its I2C slave
/// address, HID descriptor register, and GPIO pin, packed little-endian. When
/// setup settled on a candidate address (the one that ACKed the probe on the
/// bound bus), that record is preferred: multi-SKU firmware declares several
/// pads and only one is fitted, so "the first record" can be a phantom. An
/// empty body means the firmware declared none and the HID driver should probe.
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = if n > 0 { core::cmp::min(n as usize, MAX_DEVICES) } else { 0 };
    let mut first: Option<&DeviceRecord> = None;
    let mut matched: Option<&DeviceRecord> = None;
    for r in &buf[..count] {
        if r.bus_kind != BUS_KIND_ACPI || r.class != CLASS_I2C_HID || r.vendor == 0 {
            continue;
        }
        if first.is_none() {
            first = Some(r);
        }
        if driver.bound_addr != 0 && (r.vendor & 0x7F) as u8 == driver.bound_addr {
            matched = Some(r);
            break;
        }
    }
    if let Some(r) = matched.or(first) {
        let mut body = [0u8; 6];
        body[0..2].copy_from_slice(&r.vendor.to_le_bytes());
        body[2..4].copy_from_slice(&r.device.to_le_bytes());
        body[4..6].copy_from_slice(&(r.irq_source as u16).to_le_bytes());
        let _ = respond::send(sender_pid, req, E_OK, &body, out);
        return;
    }
    let _ = respond::send(sender_pid, req, E_OK, &[], out);
}
