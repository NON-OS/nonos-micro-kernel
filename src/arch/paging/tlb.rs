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

//! TLB invalidation. x86_64 drops a single translation with `invlpg` and the
//! whole non-global set by rewriting CR3; aarch64 issues the inner-shareable
//! `tlbi` variants, which the arch layer already wraps with the `dsb`/`isb`
//! barriers the architecture requires around them.

/// Drop the translation for one virtual page.
#[inline]
pub fn invalidate_page(va: u64) {
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: invlpg only invalidates a translation; it never dereferences
        // the address, so an unmapped `va` is harmless.
        unsafe {
            core::arch::asm!("invlpg [{}]", in(reg) va, options(nostack, preserves_flags));
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::mmu::flush_tlb_page(va);
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = va;
    }
}

/// Drop every non-global translation.
#[inline]
pub fn invalidate_all() {
    #[cfg(target_arch = "x86_64")]
    {
        // Rewriting CR3 with the value it already holds flushes the non-global
        // entries and leaves the active table untouched.
        // SAFETY: the value read back is the live table; writing it is a no-op
        // for translation and a flush for the TLB.
        unsafe {
            let cr3: u64;
            core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nostack, preserves_flags));
            core::arch::asm!("mov cr3, {}", in(reg) cr3, options(nostack, preserves_flags));
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::mmu::flush_tlb_all();
    }
}
