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

//! MAIR_EL1: eight one-byte slots, one per memory type a descriptor can name.

use core::arch::asm;

use super::super::attributes::MemoryType;

/// Assemble the register from the memory-type table itself, so a slot can
/// never drift from the index a descriptor writes into `AttrIndx`. A slot no
/// type claims stays zero, which reads as Device-nGnRnE; nothing selects those
/// slots, and the strictest possible answer is the right default for a slot
/// that should never be reached.
fn mair_value() -> u64 {
    let mut value = 0u64;
    let mut i = 0;
    while i < MemoryType::ALL.len() {
        let ty = MemoryType::ALL[i];
        value |= (ty.mair_attr() as u64) << (ty.attr_index() * 8);
        i += 1;
    }
    value
}

pub(in crate::arch::aarch64::mmu) fn configure_mair() {
    let mair = mair_value();
    // SAFETY: writing MAIR_EL1 at EL1 is permitted and only reinterprets the
    // attribute slots. The `isb` makes the new meaning visible to the table
    // walker before any descriptor written afterwards is used.
    unsafe {
        asm!("msr mair_el1, {0}", "isb", in(reg) mair, options(nomem, nostack, preserves_flags));
    }
}
