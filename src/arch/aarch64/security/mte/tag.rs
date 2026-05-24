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

pub fn generate_tag() -> u8 {
    let tag: u64;
    unsafe {
        asm!("irg {0}, {0}", inout(reg) 0u64 => tag);
    }
    ((tag >> 56) & 0xF) as u8
}

pub fn set_tag(ptr: *mut u8, tag: u8) {
    let tagged_ptr = (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF) | ((tag as u64) << 56);
    unsafe {
        asm!("stg {0}, [{0}]", in(reg) tagged_ptr);
    }
}

pub fn check_tag(ptr: *const u8) -> bool {
    let result: u64;
    unsafe {
        asm!("ldg {0}, [{0}]", inout(reg) ptr as u64 => result);
    }
    (result >> 56) & 0xF == (ptr as u64 >> 56) & 0xF
}
