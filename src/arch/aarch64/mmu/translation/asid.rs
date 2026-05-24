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

pub fn current_asid() -> u16 {
    let ttbr: u64;
    unsafe {
        asm!("mrs {}, ttbr0_el1", out(reg) ttbr, options(nostack));
    }
    (ttbr >> 48) as u16
}

pub fn set_asid(asid: u16) {
    let mut ttbr: u64;
    unsafe {
        asm!("mrs {}, ttbr0_el1", out(reg) ttbr, options(nostack));
    }
    ttbr = (ttbr & 0x0000_FFFF_FFFF_FFFF) | ((asid as u64) << 48);
    unsafe {
        asm!("msr ttbr0_el1, {}", "isb", in(reg) ttbr, options(nostack));
    }
}
