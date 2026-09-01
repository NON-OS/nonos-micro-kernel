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

use super::metrics::Sample;
use super::ring::SparkRing;
use super::sample::poll;

/// `tick_interval_ms` is 30, so a whole second of ticks separates two reads of
/// the process table and the rail never samples at frame rate.
const POLL_TICKS: u32 = 34;

#[derive(Clone, Copy)]
pub struct Rail {
    pub sample: Sample,
    pub spark: SparkRing,
    ticks: u32,
    warm: bool,
}

impl Rail {
    pub const fn new() -> Self {
        Rail { sample: Sample::EMPTY, spark: SparkRing::new(), ticks: 0, warm: false }
    }

    pub fn tick(&mut self) -> bool {
        self.ticks += 1;
        if self.ticks < POLL_TICKS {
            return false;
        }
        self.ticks = 0;
        let next = poll(&self.sample);
        if self.warm {
            self.spark.push(next.cpu_pct.min(100) as u8);
        }
        self.warm = true;
        self.sample = next;
        true
    }
}
