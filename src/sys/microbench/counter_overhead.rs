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

/// What one measurement costs, so every other case can have it removed.
///
/// Reading the counter twice back to back is the floor: any case measured
/// with the same pair carries this cost, and reporting a figure smaller than
/// its own instrument is how a suite ends up claiming the impossible.
pub fn measure(rounds: usize) -> u64 {
    let mut sample = Sample::new();
    for _ in 0..rounds.min(Sample::CAPACITY) {
        let start = crate::arch::read_time_counter();
        let end = crate::arch::read_time_counter();
        sample.push(end.wrapping_sub(start));
    }
    sample.sort();
    sample.quantile(1, 2)
}
