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

//! Adopt the BSP's live LAPIC state before any IPI is sent. The boot path
//! initialises the LAPIC through `sys::apic` (mode detection, MMIO mapping,
//! SVR/LVT programming); this module's own state would otherwise still be
//! zero, and the first ICR write would dereference offset 0x300 of a null
//! base. Copying the proven mode and base keeps one source of truth for how
//! the current CPU's LAPIC is reached, without touching the hardware again.

use core::sync::atomic::Ordering;

use super::error::{ApicError, ApicResult};
use super::state::{CACHED_ID, INITIALIZED, MMIO_BASE, X2APIC_MODE};

pub fn adopt_bsp_state() -> ApicResult<()> {
    if INITIALIZED.load(Ordering::Acquire) {
        return Ok(());
    }
    let (x2, base) = crate::sys::apic::lapic_state().ok_or(ApicError::NotInitialized)?;
    if !x2 && base == 0 {
        return Err(ApicError::NotInitialized);
    }
    X2APIC_MODE.store(x2, Ordering::Release);
    if !x2 {
        MMIO_BASE.store(base, Ordering::Release);
    }
    INITIALIZED.store(true, Ordering::Release);
    Ok(())
}

/// Cache this CPU's real LAPIC ID into the arch APIC module without touching
/// any other arch APIC state (mode, base, init flag). The IOAPIC redirection
/// destination is read from here; leaving it at the default 0 only works
/// where the BSP happens to be APIC 0 (QEMU), so real hardware with a nonzero
/// BSP ID drops every device interrupt. The ID is sourced from the already
/// initialised sys::apic so this has no effect on the timer or IPI paths.
pub fn cache_bsp_apic_id() {
    if let Some(id) = crate::sys::apic::local_apic_id() {
        CACHED_ID.store(id, Ordering::Release);
    }
}
