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

use nonos_libc::{DeviceRecord, BAR_KIND_PIO};

pub fn first_pio_bar(r: &DeviceRecord) -> Option<u8> {
    for i in 0..core::cmp::min(r.bar_count as usize, r.bars.len()) {
        if r.bars[i].kind == BAR_KIND_PIO && r.bars[i].size >= 0x60 {
            return Some(i as u8);
        }
    }
    None
}
