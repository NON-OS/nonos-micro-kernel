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

use core::sync::atomic::Ordering;
use sha3::{Digest, Sha3_256};

use super::super::constants::*;
use super::hwrng::{cpu_entropy64, cpu_random64, has_cpu_entropy, has_cpu_random};
use super::state::ENTROPY_POOL;
use crate::arch::read_time_counter;

pub(super) fn secure_hash(data: &[u8]) -> [u8; HASH_OUTPUT_SIZE] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub(super) fn collect_entropy() -> u64 {
    let mut entropy = ENTROPY_POOL.load(Ordering::Relaxed);

    // Jitter across a fixed spin: how many cycles the loop actually costs
    // varies with cache, frequency and interrupt arrivals.
    let before = read_time_counter();
    for _ in 0..ENTROPY_SPIN_ITERATIONS {
        core::hint::spin_loop();
    }
    let after = read_time_counter();
    entropy ^= before.wrapping_mul(after);

    // The CPU identification registers used to be mixed in here. They hold the
    // same value on every boot of a given machine, so they contributed no
    // entropy to an XOR pool; the counter jitter above and the hardware
    // generator below are the sources that actually vary.

    if has_cpu_random() {
        if let Some(hw_rng) = cpu_random64() {
            entropy ^= hw_rng;
        }
    }
    if has_cpu_entropy() {
        if let Some(hw_rng) = cpu_entropy64() {
            entropy ^= hw_rng;
        }
    }

    entropy = entropy.wrapping_mul(ENTROPY_MIX_MULTIPLIER);
    ENTROPY_POOL.store(entropy, Ordering::Relaxed);
    entropy
}
