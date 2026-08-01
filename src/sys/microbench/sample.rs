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

/// A fixed run of timings, held without allocating.
///
/// The heap is not up when the earliest cases run, and a microbenchmark that
/// allocates measures the allocator as much as the thing under test.
pub struct Sample {
    ticks: [u64; Sample::CAPACITY],
    used: usize,
}

impl Sample {
    pub const CAPACITY: usize = 1024;

    pub const fn new() -> Self {
        Self { ticks: [0; Self::CAPACITY], used: 0 }
    }

    pub fn push(&mut self, ticks: u64) {
        if self.used < Self::CAPACITY {
            self.ticks[self.used] = ticks;
            self.used += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.used
    }

    /// Sort in place so the quantiles below are reads rather than searches.
    /// Insertion sort: the run is small, and a recursive sort would put an
    /// unbounded frame on a kernel stack.
    pub fn sort(&mut self) {
        let mut i = 1;
        while i < self.used {
            let value = self.ticks[i];
            let mut j = i;
            while j > 0 && self.ticks[j - 1] > value {
                self.ticks[j] = self.ticks[j - 1];
                j -= 1;
            }
            self.ticks[j] = value;
            i += 1;
        }
    }

    pub fn quantile(&self, numerator: usize, denominator: usize) -> u64 {
        if self.used == 0 {
            return 0;
        }
        let at = (self.used - 1) * numerator / denominator;
        self.ticks[at]
    }

    pub fn min(&self) -> u64 {
        if self.used == 0 { 0 } else { self.ticks[0] }
    }

    pub fn max(&self) -> u64 {
        if self.used == 0 { 0 } else { self.ticks[self.used - 1] }
    }
}
