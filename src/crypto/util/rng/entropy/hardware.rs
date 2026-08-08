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

//! The CPU-provided entropy inputs this pool draws on.
//!
//! The retry budgets that used to live here now belong to the arch backend,
//! which knows what its own generator's busy behaviour looks like.

use crate::arch::cpu_random;

/// Conditioned DRBG output from the CPU, if it has that tap.
pub fn cpu_random64() -> Option<u64> {
    cpu_random::random_u64()
}

/// Reseeded entropy from the CPU, if it has that tap.
pub fn cpu_entropy64() -> Option<u64> {
    cpu_random::entropy_u64()
}

/// True iff the CPU can produce conditioned DRBG output.
pub fn has_cpu_random() -> bool {
    cpu_random::random_available()
}

/// True iff the CPU can produce reseeded entropy.
pub fn has_cpu_entropy() -> bool {
    cpu_random::entropy_available()
}

/// The free-running cycle counter. Sampled for timing jitter, never for time.
pub(crate) fn read_cycle_counter() -> u64 {
    crate::arch::read_time_counter()
}
