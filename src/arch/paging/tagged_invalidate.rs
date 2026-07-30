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

//! Retiring one address space's translations, leaving every other space's
//! entries in the TLB.
//!
//! Both architectures tag translations with an address-space id, a PCID on
//! x86_64 and an ASID on aarch64, and both can invalidate by that tag. It is the
//! difference between a context switch costing one address space's entries and
//! costing every non-global entry on the core, so it is worth naming rather than
//! falling back to a full flush. Callers must ask
//! [`supports_tagged_invalidation`](super::supports_tagged_invalidation) first,
//! because a part without the feature cannot do this at all and needs the full
//! flush instead.
//!
//! [`supports_tagged_invalidation`]: super::supports_tagged_invalidation

/// Invalidate every translation tagged with `asid`, on a part that supports it.
///
/// Does nothing where the feature is absent, so a caller that skipped the
/// capability check gets a no-op rather than an undefined instruction; that
/// leaves stale entries, which is why the check is the caller's job and is
/// documented as such rather than hidden here.
#[inline]
pub fn invalidate_tagged(asid: u16) {
    if !super::supports_tagged_invalidation() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    {
        // INVPCID descriptor: the id in the low 64 bits, the address in the
        // high ones, unused for a whole-context invalidation.
        let descriptor: [u64; 2] = [asid as u64, 0];
        // SAFETY: type 1 is single-context invalidation, which only retires TLB
        // entries and never dereferences the address half of the descriptor. The
        // descriptor is a live local of exactly the layout the instruction reads.
        unsafe {
            core::arch::asm!(
                "invpcid {}, [{}]",
                in(reg) 1u64,
                in(reg) descriptor.as_ptr(),
                options(nostack)
            );
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        // TLBI ASIDE1IS takes the ASID in bits 63:48 of its operand and shoots
        // it down across the inner shareable domain, so other cores drop the
        // same entries. DSB first so earlier table writes are visible to that
        // broadcast, ISB after so this core does not run on a stale translation.
        let operand = (asid as u64) << 48;
        // SAFETY: an invalidate-by-ASID touches no memory and faults nowhere; the
        // surrounding barriers are the sequence the architecture requires.
        unsafe {
            core::arch::asm!(
                "dsb ishst",
                "tlbi aside1is, {}",
                "dsb ish",
                "isb",
                in(reg) operand,
                options(nostack, preserves_flags)
            );
        }
    }
}
