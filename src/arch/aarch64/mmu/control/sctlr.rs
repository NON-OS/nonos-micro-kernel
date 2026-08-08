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

//! SCTLR_EL1: the switch itself.

use core::arch::asm;

/// Enable the EL1&0 stage 1 MMU.
const SCTLR_M: u64 = 1 << 0;
/// Data accesses are cacheable rather than forced Non-cacheable.
const SCTLR_C: u64 = 1 << 2;
/// Instruction fetches are cacheable.
const SCTLR_I: u64 = 1 << 12;

pub(in crate::arch::aarch64::mmu) fn enable_mmu() {
    // SAFETY: the caller has written a complete set of translation tables and
    // installed TTBR0/TTBR1, MAIR and TCR. The barriers below publish those
    // writes to the table walker and drop any stale entry firmware left behind
    // before the walker is switched on; the `isb` after the write makes the
    // enable take effect before the next instruction is fetched.
    unsafe {
        asm!(
            // Table writes must be observable by the walker first.
            "dsb ishst",
            // Firmware ran with its own tables; none of its entries are valid
            // for ours.
            "tlbi vmalle1",
            "dsb nsh",
            "isb",
            options(nostack, preserves_flags)
        );

        let mut sctlr: u64;
        asm!("mrs {}, sctlr_el1", out(reg) sctlr, options(nomem, nostack, preserves_flags));
        sctlr |= SCTLR_M | SCTLR_C | SCTLR_I;
        asm!("msr sctlr_el1, {}", "isb", in(reg) sctlr, options(nostack, preserves_flags));
    }
}
