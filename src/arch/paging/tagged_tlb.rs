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

//! Can this CPU drop one address space's translations and leave the rest?
//!
//! Both architectures tag TLB entries with the address space that made them,
//! x86_64 as a PCID and aarch64 as an ASID, which is what lets a context
//! switch keep the outgoing entries instead of flushing everything. They
//! differ in whether the tag can be relied on: PCID and the `invpcid`
//! instruction that makes it useful are both optional and have to be asked
//! for, while ASID has been in the architecture since ARMv6 and the
//! `tlbi aside1is` that uses it is always there.
//!
//! A caller that gets `false` is not broken, only slower: it falls back to
//! flushing the whole non-global set.

/// Whether translations can be dropped per address space rather than wholesale.
#[inline]
pub fn supports_tagged_invalidation() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::paging::supports_tagged_invalidation()
    }
    #[cfg(target_arch = "aarch64")]
    {
        true
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        false
    }
}
