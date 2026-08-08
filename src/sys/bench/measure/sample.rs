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

/// What one timing run produced, in counter ticks.
///
/// `min` is the number to quote. It is the run least disturbed by an interrupt,
/// a cache miss or a migration, so it is the closest thing to the cost of the
/// work itself; `avg` carries whatever the machine was also doing, and `max` is
/// usually a timer tick landing mid-measurement rather than a property of the
/// code. Reporting all three is what lets a reader tell a real regression from
/// a noisy runner.
#[derive(Clone, Copy)]
pub struct Sample {
    pub min: u64,
    pub avg: u64,
    pub max: u64,
    pub iterations: u32,
}

impl Sample {
    /// Fold a set of tick counts into the three figures worth printing.
    pub(crate) fn from_runs(runs: &[u64]) -> Self {
        if runs.is_empty() {
            return Self { min: 0, avg: 0, max: 0, iterations: 0 };
        }

        let mut min = u64::MAX;
        let mut max = 0u64;
        // Widened, because a slow operation timed many times overflows a u64
        // sum of cycle counts far sooner than the arithmetic suggests.
        let mut total: u128 = 0;

        for &run in runs {
            if run < min {
                min = run;
            }
            if run > max {
                max = run;
            }
            total += run as u128;
        }

        Self { min, avg: (total / runs.len() as u128) as u64, max, iterations: runs.len() as u32 }
    }

    /// Ticks converted to nanoseconds, or `None` where the platform never
    /// reported a counter frequency and the conversion would be invented.
    pub fn min_nanos(&self) -> Option<u64> {
        let hz = crate::arch::time_counter_hz();
        if hz == 0 {
            return None;
        }
        Some(((self.min as u128 * 1_000_000_000u128) / hz as u128) as u64)
    }
}
