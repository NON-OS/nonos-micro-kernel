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

pub fn set_ttbr0(addr: u64, asid: u16) {
    let ttbr = addr | ((asid as u64) << 48);
    unsafe {
        asm!("msr ttbr0_el1, {0}", "isb", in(reg) ttbr, options(nostack));
    }
}

pub fn set_ttbr1(addr: u64) {
    unsafe {
        asm!("msr ttbr1_el1, {0}", "isb", in(reg) addr, options(nostack));
    }
}

pub fn read_ttbr0() -> u64 {
    let ttbr: u64;
    unsafe {
        asm!("mrs {}, ttbr0_el1", out(reg) ttbr, options(nostack));
    }
    ttbr
}

pub fn read_ttbr1() -> u64 {
    let ttbr: u64;
    unsafe {
        asm!("mrs {}, ttbr1_el1", out(reg) ttbr, options(nostack));
    }
    ttbr
}
