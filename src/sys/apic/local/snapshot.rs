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

//! Read-only snapshot of the live LAPIC surface, for the SMP bring-up path.
//! This module is the single owner of the LAPIC mode and register base; the
//! arch-side IPI code adopts these values instead of re-deriving them, so both
//! stacks always agree on how the current CPU's LAPIC is reached.

use core::sync::atomic::Ordering;

use super::state::{LAPIC_BASE, LAPIC_INIT, LAPIC_X2};

/// `(x2apic_mode, register_base)` once the LAPIC is initialised, else None.
/// In x2APIC mode the base is meaningless (registers are MSRs) and callers
/// must not dereference it.
pub fn lapic_state() -> Option<(bool, u64)> {
    if !LAPIC_INIT.load(Ordering::Acquire) {
        return None;
    }
    Some((LAPIC_X2.load(Ordering::Acquire), LAPIC_BASE.load(Ordering::Acquire)))
}

/// This CPU's LAPIC ID, read through the proven register path (MMIO in
/// xAPIC mode, the APIC-ID MSR in x2APIC mode). Returns None before the
/// LAPIC is initialised. The xAPIC register carries the ID in bits 24..31;
/// the x2APIC MSR carries the full 32-bit ID with no shift.
pub fn local_apic_id() -> Option<u32> {
    if !LAPIC_INIT.load(Ordering::Acquire) {
        return None;
    }
    let raw = unsafe { super::regs::lapic_read_raw(super::constants::LAPIC_ID) };
    if LAPIC_X2.load(Ordering::Acquire) {
        Some(raw)
    } else {
        Some(raw >> 24)
    }
}
