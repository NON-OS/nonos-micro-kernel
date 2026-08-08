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

use super::sample::Sample;
use crate::arch::read_time_counter;

/// Runs discarded before timing starts, so the first pass through cold code
/// and cold caches is not reported as the cost of the operation.
const WARMUP: u32 = 8;

/// Ceiling on stored runs. A microbenchmark that wants more than this is
/// measuring throughput rather than latency and belongs in its own harness.
const MAX_RUNS: usize = 256;

/// Time `body` once per iteration and report the spread.
///
/// Interrupts stay enabled on purpose. Masking them would produce prettier
/// numbers that no caller ever experiences, and the kernel's own paths run
/// with interrupts live; `min` across enough runs already gives the
/// undisturbed cost without lying about the environment.
///
/// The counter is read through the arch boundary, so this measures a TSC on
/// x86_64 and `CNTPCT_EL0` on aarch64 without knowing which.
pub fn measure(iterations: u32, mut body: impl FnMut()) -> Sample {
    for _ in 0..WARMUP {
        body();
    }

    let wanted = (iterations as usize).min(MAX_RUNS);
    let mut runs = [0u64; MAX_RUNS];

    for slot in runs.iter_mut().take(wanted) {
        let start = read_time_counter();
        body();
        let end = read_time_counter();
        *slot = end.saturating_sub(start);
    }

    Sample::from_runs(&runs[..wanted])
}
