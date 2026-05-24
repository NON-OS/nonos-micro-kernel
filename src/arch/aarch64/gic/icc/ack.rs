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

pub fn acknowledge_interrupt() -> Option<u32> {
    let iar: u64;
    unsafe {
        asm!("mrs {0}, icc_iar1_el1", out(reg) iar, options(nostack));
    }
    let intid = (iar & 0xFF_FFFF) as u32;
    if intid >= 1020 {
        None
    } else {
        Some(intid)
    }
}

pub fn end_interrupt(intid: u32) {
    unsafe {
        asm!("msr icc_eoir1_el1, {0}", "isb", in(reg) intid as u64, options(nostack));
    }
}

pub fn drop_priority(intid: u32) {
    unsafe {
        asm!("msr icc_dir_el1, {0}", in(reg) intid as u64, options(nostack));
    }
}
