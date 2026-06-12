// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use core::sync::atomic::{AtomicU64, Ordering};

static JITTER_SEED: AtomicU64 = AtomicU64::new(0);

pub fn init_jitter(entropy: u64) {
    JITTER_SEED.store(entropy, Ordering::Release);
}

pub(super) fn next_jitter() -> u64 {
    let seed = JITTER_SEED.load(Ordering::Acquire);
    let next = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    JITTER_SEED.store(next, Ordering::Release);
    next
}

pub fn add_random_delay() {
    let jitter = next_jitter() & 0xFFFF;
    for _ in 0..jitter {
        core::hint::spin_loop();
    }
}

pub fn add_fixed_delay(iterations: u32) {
    for _ in 0..iterations {
        core::hint::spin_loop();
    }
}
