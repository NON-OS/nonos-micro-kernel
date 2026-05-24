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


const CPACR_FPEN_FULL: u64 = 0b11 << 20;
const CPACR_FPEN_TRAP: u64 = 0b00 << 20;
const CPACR_FPEN_MASK: u64 = 0b11 << 20;

pub fn enable() {
    let mut cpacr: u64;
    unsafe {
        asm!("mrs {}, cpacr_el1", out(reg) cpacr, options(nomem, nostack));
    }
    cpacr = (cpacr & !CPACR_FPEN_MASK) | CPACR_FPEN_FULL;
    unsafe {
        asm!("msr cpacr_el1, {}", "isb", in(reg) cpacr, options(nomem, nostack));
    }
}

pub fn disable() {
    let mut cpacr: u64;
    unsafe {
        asm!("mrs {}, cpacr_el1", out(reg) cpacr, options(nomem, nostack));
    }
    cpacr = (cpacr & !CPACR_FPEN_MASK) | CPACR_FPEN_TRAP;
    unsafe {
        asm!("msr cpacr_el1, {}", "isb", in(reg) cpacr, options(nomem, nostack));
    }
}
