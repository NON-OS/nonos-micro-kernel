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

use super::pool::EntropyPool;

pub(super) struct TimingSeed {
    pub(super) tsc_start: u64,
}

pub(super) fn collect(pool: &mut EntropyPool) -> TimingSeed {
    let tsc_start = super::super::platform::read_cycle_counter();
    pool.push_u64(tsc_start);

    for i in 0..16 {
        collect_jitter_sample(pool, i);
    }

    TimingSeed { tsc_start }
}

fn collect_jitter_sample(pool: &mut EntropyPool, sample: u32) {
    let pit = super::super::platform::read_second_clock();
    let tsc_before = super::super::platform::read_cycle_counter();
    let delay = ((pit & 0x3F) as u32) + (sample + 1) * 5;

    for _ in 0..delay {
        core::hint::spin_loop();
    }

    let jitter = super::super::platform::read_cycle_counter().wrapping_sub(tsc_before);
    let mixed = (pit as u64) ^ jitter.rotate_left((sample % 63) + 1);
    pool.push_u64(mixed);
}
