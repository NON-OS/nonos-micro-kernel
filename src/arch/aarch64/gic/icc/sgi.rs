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

//! Software-generated interrupts: the GIC's inter-processor doorbell.
//!
//! `ICC_SGI1R_EL1` addresses a target by affinity, not by CPU number. Aff1,
//! Aff2 and Aff3 select a cluster and the 16-bit target list picks cores inside
//! it by Aff0, so a core whose Aff0 is 16 or higher cannot be named by a
//! single write. Setting `IRM` instead broadcasts to every core but the sender
//! and ignores the affinity fields entirely.

use core::arch::asm;

/// Highest INTID the SGI range holds.
const SGI_MAX: u32 = 15;

const INTID_SHIFT: u64 = 24;
const AFF1_SHIFT: u64 = 16;
const AFF2_SHIFT: u64 = 32;
const AFF3_SHIFT: u64 = 48;
/// Interrupt Routing Mode: send to all cores except this one.
const IRM_ALL_OTHERS: u64 = 1 << 40;

/// Send `intid` to the single core whose packed affinity is `target`.
pub fn send_sgi(target: u32, intid: u32) -> Result<(), ()> {
    if intid > SGI_MAX {
        return Err(());
    }
    let aff0 = target & 0xFF;
    if aff0 > 15 {
        // One write can only reach Aff0 0..15 within a cluster. Silently
        // targeting the wrong core would be worse than refusing.
        return Err(());
    }
    let value = ((intid as u64) << INTID_SHIFT)
        | (1u64 << aff0)
        | ((((target >> 8) & 0xFF) as u64) << AFF1_SHIFT)
        | ((((target >> 16) & 0xFF) as u64) << AFF2_SHIFT)
        | ((((target >> 24) & 0xFF) as u64) << AFF3_SHIFT);
    write_sgi1r(value);
    Ok(())
}

/// Send `intid` to every core except the caller.
pub fn send_sgi_all_others(intid: u32) -> Result<(), ()> {
    if intid > SGI_MAX {
        return Err(());
    }
    write_sgi1r(((intid as u64) << INTID_SHIFT) | IRM_ALL_OTHERS);
    Ok(())
}

fn write_sgi1r(value: u64) {
    // SAFETY: ICC_SGI1R_EL1 is write-only and accessible at EL1 once the CPU
    // interface is enabled, which `icc::init` did during GIC bring-up. The
    // `isb` keeps the write from being observed after later interrupt state
    // changes.
    unsafe {
        asm!("msr icc_sgi1r_el1, {0}", "isb", in(reg) value, options(nomem, nostack, preserves_flags));
    }
}
