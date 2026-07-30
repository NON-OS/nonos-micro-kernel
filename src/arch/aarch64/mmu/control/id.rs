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

//! What ID_AA64MMFR0_EL1 says this core's translation hardware can do.
//!
//! TCR_EL1 must not claim more than the core implements: an intermediate
//! physical address size larger than the core's PA range is a programming
//! error, not a wish, and a 16-bit ASID field on a core with 8-bit ASIDs
//! silently loses the top half of every address-space tag.

use core::arch::asm;

/// The largest `IPS` encoding this kernel uses. 4 KiB granule tables address
/// at most 48 bits without FEAT_LPA2, so a core advertising 52-bit physical
/// addresses is still driven at 48.
const IPS_48_BIT: u64 = 0b101;

fn read_aa64mmfr0() -> u64 {
    let value: u64;
    // SAFETY: ID_AA64MMFR0_EL1 is a read-only identification register that is
    // always readable at EL1. The read has no side effects.
    unsafe {
        asm!("mrs {}, id_aa64mmfr0_el1", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

/// `IPS` for TCR_EL1, taken from `PARange` and capped at 48 bits.
pub(super) fn intermediate_pa_size() -> u64 {
    let parange = read_aa64mmfr0() & 0xF;
    parange.min(IPS_48_BIT)
}

/// True when the core implements 16-bit ASIDs, which is what `AS` selects.
pub(super) fn has_16_bit_asid() -> bool {
    (read_aa64mmfr0() >> 4) & 0xF == 0b0010
}
