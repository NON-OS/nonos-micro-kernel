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

use nonos_libc::{mk_device_list, DeviceRecord};

use super::bar_select;
use super::found::Found;
use super::match_device;

const MAX_DEVICES: usize = 64;

pub fn find_virtio_gpu() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let limit = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..limit] {
        if !match_device::is_usable(r) {
            continue;
        }
        if let Some((bar, kind, size)) = bar_select::select(r) {
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
