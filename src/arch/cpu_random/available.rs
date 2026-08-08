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

//! Presence probes for the two CPU random taps.
//!
//! The answer cannot change while the machine runs, and the probe is a trap to
//! the hypervisor on x86_64, so each result is memoised. Entropy consumers call
//! these in tight loops when filling a pool.

use core::sync::atomic::{AtomicU8, Ordering};

const UNKNOWN: u8 = 0;
const PRESENT: u8 = 1;
const ABSENT: u8 = 2;

static RANDOM: AtomicU8 = AtomicU8::new(UNKNOWN);
static ENTROPY: AtomicU8 = AtomicU8::new(UNKNOWN);

/// True iff the CPU can produce conditioned DRBG output.
pub(crate) fn random_available() -> bool {
    memoise(&RANDOM, probe_random)
}

/// True iff the CPU can produce reseeded entropy suitable for seeding a DRBG.
pub(crate) fn entropy_available() -> bool {
    memoise(&ENTROPY, probe_entropy)
}

fn memoise(slot: &AtomicU8, probe: fn() -> bool) -> bool {
    match slot.load(Ordering::Relaxed) {
        PRESENT => true,
        ABSENT => false,
        _ => {
            let present = probe();
            slot.store(if present { PRESENT } else { ABSENT }, Ordering::Relaxed);
            present
        }
    }
}

fn probe_random() -> bool {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::cpu_random::has_rdrand();
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::cpu_random::has_rng();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return false;
}

fn probe_entropy() -> bool {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::cpu_random::has_rdseed();
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::cpu_random::has_rng();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return false;
}
