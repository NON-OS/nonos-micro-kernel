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
use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO};

use super::classify::classify;
use super::defs::{Found, MAX_DEVICES};

/// Collect every LPSS I2C host controller with a usable MMIO window into `out`,
/// returning how many were written. The touchpad may hang off any one of them,
/// so the caller probes each rather than committing to the first.
pub fn find_controllers(out: &mut [Found]) -> usize {
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
        let Some((family, clock_hz, is_acpi)) = classify(r) else { continue };
        let bar0 = r.bars[0];
        if r.bar_count != 0 && bar0.kind == BAR_KIND_MMIO && bar0.size != 0 {
            out[count] = Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                bar0_size: bar0.size,
                pci_device: r.device,
                clock_hz,
                family,
                is_acpi,
            };
            count += 1;
        }
    }
    count
}
