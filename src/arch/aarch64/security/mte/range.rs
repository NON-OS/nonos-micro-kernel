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

use core::arch::asm;

pub fn clear_tag(ptr: *mut u8, size: usize) {
    let aligned_ptr = (ptr as u64) & !0xF;
    let granules = size.saturating_add(15) / 16;
    for i in 0..granules {
        clear_granule(aligned_ptr + (i as u64 * 16));
    }
}

fn clear_granule(addr: u64) {
    unsafe {
        asm!("stzg xzr, [{0}]", in(reg) addr);
    }
}
