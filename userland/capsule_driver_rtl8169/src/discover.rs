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

mod bar_command;
mod bar_mmio;
mod support;

use nonos_libc::{mk_device_list, DeviceRecord};
use self::bar_command::command_bits;
use self::bar_mmio::first_mmio_bar;
use self::support::is_supported;

const MAX_DEVICES: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub bar_index: u8,
    pub bar_size: u64,
    pub command_bits: u16,
}

pub fn find_rtl8169() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if !is_supported(r) {
            continue;
        }
        if r.irq_pin == 0 || r.irq_line == 0xFF {
            continue;
        }
        if let Some((bar_index, bar_size)) = first_mmio_bar(r) {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                bar_index,
                bar_size,
                command_bits: command_bits(r),
            });
        }
    }
    None
}
