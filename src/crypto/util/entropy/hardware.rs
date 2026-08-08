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

use super::state::ENTROPY_SOURCE;
use crate::arch::cpu_random;

/// Record what the CPU offers, once, before anything asks for a byte.
pub fn init() {
    let random = cpu_random::random_available();
    let entropy = cpu_random::entropy_available();
    // SAFETY: single-threaded initialization during early boot
    unsafe {
        ENTROPY_SOURCE.random_available = random;
        ENTROPY_SOURCE.entropy_available = entropy;
    }
}

/// Conditioned DRBG output from the CPU, if it has one.
pub(crate) fn hardware_random64() -> Option<u64> {
    // SAFETY: reading global state after init
    if !unsafe { ENTROPY_SOURCE.random_available } {
        return None;
    }
    cpu_random::random_u64()
}

/// Reseeded entropy from the CPU, if it has a source for it.
pub(crate) fn hardware_entropy64() -> Option<u64> {
    // SAFETY: reading global state after init
    if !unsafe { ENTROPY_SOURCE.entropy_available } {
        return None;
    }
    cpu_random::entropy_u64()
}

pub fn has_hardware_rng() -> bool {
    // SAFETY: reading global state after init
    unsafe { ENTROPY_SOURCE.random_available || ENTROPY_SOURCE.entropy_available }
}
