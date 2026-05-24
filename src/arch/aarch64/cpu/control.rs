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

pub(super) fn configure_sctlr() {
    let mut sctlr: u64;
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) sctlr, options(nostack));
    }
    sctlr |= 1 << 0;
    sctlr |= 1 << 2;
    sctlr |= 1 << 12;
    sctlr |= 1 << 26;
    sctlr &= !(1 << 19);
    unsafe {
        asm!("msr sctlr_el1, {}", in(reg) sctlr, options(nostack));
        asm!("isb", options(nostack));
    }
}

pub(super) fn enable_fp_simd() {
    unsafe {
        asm!(
            "mrs x0, cpacr_el1",
            "orr x0, x0, #(3 << 20)",
            "msr cpacr_el1, x0",
            "isb",
            out("x0") _,
            options(nostack)
        );
    }
}

pub(super) fn configure_cache() {
    unsafe {
        asm!("ic iallu", options(nostack));
        asm!("dsb ish", options(nostack));
        asm!("isb", options(nostack));
    }
}
