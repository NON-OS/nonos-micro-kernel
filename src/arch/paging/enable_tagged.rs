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

//! Turning on address-space tagged translations.
//!
//! x86_64 starts with tagging off and the low bits of the root register meaning
//! something else, so it has to be switched on deliberately and only after the
//! part is known to support it. aarch64 carries the ASID in the root register
//! from reset, so there is nothing to enable and asking is already answered.
//! Callers get the same yes or no either way and do not have to know which
//! situation they are in.

/// Turn on tagged translations, and report whether they are now in use.
///
/// False means this core will flush every non-global entry on an address-space
/// switch, which is correct but slower; it is never an error to carry on.
#[inline]
pub fn enable_tagged_invalidation() -> bool {
    if !super::supports_tagged_invalidation() {
        return false;
    }
    #[cfg(target_arch = "x86_64")]
    {
        // CR4.PCIDE, bit 17. The architecture requires CR3[11:0] to be zero at
        // the moment this is set, which holds here because the kernel's own
        // address space is installed with PCID 0 and this runs before any other
        // space exists.
        const CR4_PCIDE: u64 = 1 << 17;
        let cr4: u64;
        // SAFETY: reading CR4 has no side effect; setting PCIDE only changes how
        // the low bits of the root register are interpreted from here on.
        unsafe {
            core::arch::asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack));
            core::arch::asm!("mov cr4, {}", in(reg) cr4 | CR4_PCIDE, options(nostack));
        }
        true
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // The ASID is part of the root register from reset, so support and use
        // are the same question and it was answered above.
        true
    }
}
