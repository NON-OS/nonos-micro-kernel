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

//! TCR_EL1: the shape of both translation regimes.
//!
//! Both halves get 48-bit address spaces on a 4 KiB granule, walked through
//! write-back inner-shareable cacheable tables. The granule fields are the easy
//! ones to get wrong: TG0 spells 4 KiB as `0b00` while TG1 spells the same size
//! as `0b10`, and a wrong granule makes every table walk read the wrong bits of
//! the address.

use core::arch::asm;

use super::id;

/// 64 - 48: both regimes translate 48-bit virtual addresses.
const ADDRESS_SIZE_SHIFT: u64 = 16;

/// Normal memory, write-back, read-allocate write-allocate. Used for the
/// cacheability of the table walks themselves, inner and outer, in both halves.
const RGN_WB_RA_WA: u64 = 0b01;
/// Inner shareable table walks, matching how the tables are mapped.
const SH_INNER: u64 = 0b11;
/// TTBR0 regime, 4 KiB granule.
const TG0_4K: u64 = 0b00;
/// TTBR1 regime, 4 KiB granule. Deliberately a different encoding from `TG0`.
const TG1_4K: u64 = 0b10;

const T0SZ_SHIFT: u64 = 0;
const IRGN0_SHIFT: u64 = 8;
const ORGN0_SHIFT: u64 = 10;
const SH0_SHIFT: u64 = 12;
const TG0_SHIFT: u64 = 14;
const T1SZ_SHIFT: u64 = 16;
const IRGN1_SHIFT: u64 = 24;
const ORGN1_SHIFT: u64 = 26;
const SH1_SHIFT: u64 = 28;
const TG1_SHIFT: u64 = 30;
const IPS_SHIFT: u64 = 32;
const AS_16_BIT: u64 = 1 << 36;
const TBI0: u64 = 1 << 37;
const TBI1: u64 = 1 << 38;

fn tcr_value() -> u64 {
    let mut tcr = (ADDRESS_SIZE_SHIFT << T0SZ_SHIFT)
        | (RGN_WB_RA_WA << IRGN0_SHIFT)
        | (RGN_WB_RA_WA << ORGN0_SHIFT)
        | (SH_INNER << SH0_SHIFT)
        | (TG0_4K << TG0_SHIFT)
        | (ADDRESS_SIZE_SHIFT << T1SZ_SHIFT)
        | (RGN_WB_RA_WA << IRGN1_SHIFT)
        | (RGN_WB_RA_WA << ORGN1_SHIFT)
        | (SH_INNER << SH1_SHIFT)
        | (TG1_4K << TG1_SHIFT)
        | (id::intermediate_pa_size() << IPS_SHIFT)
        // The top byte of an address is not part of translation, which is what
        // lets tagged pointers survive a dereference.
        | TBI0
        | TBI1;

    // `A1` stays clear, so TTBR0_EL1 holds the ASID for both regimes. The
    // paging boundary masks TTBR0 on that assumption.
    if id::has_16_bit_asid() {
        tcr |= AS_16_BIT;
    }
    tcr
}

pub(in crate::arch::aarch64::mmu) fn configure_tcr() {
    let tcr = tcr_value();
    // SAFETY: writing TCR_EL1 at EL1 is permitted. It only takes effect for
    // walks started after the `isb`, and the MMU is still off here.
    unsafe {
        asm!("msr tcr_el1, {0}", "isb", in(reg) tcr, options(nomem, nostack, preserves_flags));
    }
}
