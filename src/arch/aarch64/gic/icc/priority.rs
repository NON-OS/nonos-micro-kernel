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

pub fn set_priority_mask(priority: u8) {
    unsafe {
        asm!("msr icc_pmr_el1, {0}", "isb", in(reg) priority as u64, options(nostack));
    }
}

pub fn running_priority() -> u8 {
    let rpr: u64;
    unsafe {
        asm!("mrs {0}, icc_rpr_el1", out(reg) rpr, options(nostack));
    }
    rpr as u8
}

pub fn highest_pending_priority() -> u8 {
    let hppir: u64;
    unsafe {
        asm!("mrs {0}, icc_hppir1_el1", out(reg) hppir, options(nostack));
    }
    (hppir >> 24) as u8
}
