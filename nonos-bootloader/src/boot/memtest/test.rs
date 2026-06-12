// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::constants::{PATTERN_5, PATTERN_A};

pub fn test_region(addr: u64, pages: u64) -> u32 {
    let test_pages = pages.min(16);
    let mut errors = 0u32;
    for p in 0..test_pages {
        let ptr = (addr + p * 4096) as *mut u64;
        errors += test_location(ptr);
    }
    errors
}

fn test_location(ptr: *mut u64) -> u32 {
    unsafe {
        let backup = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, PATTERN_A);
        let r1 = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, PATTERN_5);
        let r2 = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, backup);
        if r1 != PATTERN_A || r2 != PATTERN_5 {
            1
        } else {
            0
        }
    }
}
