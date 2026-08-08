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

//! The two reads themselves.
//!
//! Both gate on the presence probe first: on x86_64 an unsupported `RDRAND`
//! raises #UD, and on aarch64 an unsupported `RNDR` is an undefined system
//! register access, so the check is a safety precondition rather than an
//! optimisation. The per-arch backend owns the retry budget, because how long
//! a generator is allowed to stay busy is a property of that generator.

use super::available::{entropy_available, random_available};

/// One conditioned 64-bit draw, or `None` if the generator is absent or stayed
/// busy for the whole retry budget.
pub(crate) fn random_u64() -> Option<u64> {
    if !random_available() {
        return None;
    }
    // SAFETY: `random_available` confirmed the CPU implements the instruction,
    // which is the backend's only precondition.
    #[cfg(target_arch = "x86_64")]
    return unsafe { crate::arch::x86_64::cpu_random::rdrand_u64() };
    #[cfg(target_arch = "aarch64")]
    return unsafe { crate::arch::aarch64::cpu_random::rndr_u64() };
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return None;
}

/// One reseeded 64-bit draw, or `None` under the same conditions. Callers
/// seeding a software DRBG want this one; it is slower and fails more often.
pub(crate) fn entropy_u64() -> Option<u64> {
    if !entropy_available() {
        return None;
    }
    // SAFETY: `entropy_available` confirmed the CPU implements the
    // instruction, which is the backend's only precondition.
    #[cfg(target_arch = "x86_64")]
    return unsafe { crate::arch::x86_64::cpu_random::rdseed_u64() };
    #[cfg(target_arch = "aarch64")]
    return unsafe { crate::arch::aarch64::cpu_random::rndrrs_u64() };
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return None;
}
