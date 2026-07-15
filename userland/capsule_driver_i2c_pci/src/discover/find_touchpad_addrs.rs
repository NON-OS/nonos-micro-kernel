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

use super::defs::{AcpiTouchpad, CLASS_I2C_HID, MAX_DEVICES};

/// Collect every ACPI-declared i2c-HID device into `out`, returning how many
/// were written. Firmware for a chassis with several possible pads (HP ships
/// ELAN2513 and ELAN0712 variants of the same laptop) declares them all and
/// gates the real one behind `_STA`, which this kernel does not evaluate — so
/// the caller must treat each entry as a candidate and let the bus probe
/// decide, never just the first record.
pub fn find_touchpad_addrs(out: &mut [AcpiTouchpad]) -> usize {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if count >= out.len() {
            break;
        }
        if r.bus_kind == BUS_KIND_ACPI && r.class == CLASS_I2C_HID && r.vendor != 0 {
            out[count] = AcpiTouchpad {
                addr: (r.vendor & 0x7F) as u8,
                controller_idx: r.pci_progif.checked_sub(1),
            };
            count += 1;
        }
    }
    count
}
