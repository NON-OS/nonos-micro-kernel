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

use super::constants::MAX_DEVICES;
use super::first_register_bar::first_register_bar;
use super::found::Found;
use super::is_match::is_match;

pub fn find_virtio_net() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    for r in &buf[..count] {
        if !is_match(r) || r.irq_pin == 0 || r.irq_line == 0xFF {
            continue;
        }
        if let Some((idx, kind, size)) = first_register_bar(r) {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                register_bar: idx,
                register_kind: kind,
                register_size: size,
            });
        }
    }
    None
}
