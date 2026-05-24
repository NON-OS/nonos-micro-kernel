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

pub(super) fn configure_mair() {
    let mair = (0x00 << 0) | (0x04 << 8) | (0x0C << 16) | (0x44 << 24) | (0xFF << 32) | (0xBB << 40);
    unsafe {
        asm!("msr mair_el1, {0}", "isb", in(reg) mair, options(nostack));
    }
}

pub(super) fn configure_tcr() {
    let tcr = (16 << 0)
        | (0b11 << 10)
        | (0b01 << 12)
        | (0b1 << 14)
        | (16 << 16)
        | (0b11 << 26)
        | (0b01 << 28)
        | (0b1 << 30)
        | (0b10 << 32)
        | (0b1 << 36)
        | (0b1 << 37)
        | (0b1 << 38)
        | (0b1 << 39);
    unsafe {
        asm!("msr tcr_el1, {0}", "isb", in(reg) tcr, options(nostack));
    }
}

pub(super) fn enable_mmu() {
    unsafe {
        asm!("dsb sy", "isb", options(nostack));
        let mut sctlr: u64;
        asm!("mrs {}, sctlr_el1", out(reg) sctlr, options(nostack));
        sctlr |= 1 << 0;
        sctlr |= 1 << 2;
        sctlr |= 1 << 12;
        asm!("msr sctlr_el1, {}", "isb", in(reg) sctlr, options(nostack));
    }
}
