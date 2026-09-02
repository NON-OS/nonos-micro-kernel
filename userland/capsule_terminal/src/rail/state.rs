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

use super::disk_query::query as disk_usage;
use super::metrics::Sample;
use super::net_query::query;
use super::ring::SparkRing;
use super::sample::poll;

/// `tick_interval_ms` is 30, so a whole second of ticks separates two reads of
/// the process table and the rail never samples at frame rate.
const POLL_TICKS: u32 = 34;

/// The lease query is an IPC round trip that can spend its whole timeout, so it
/// runs once in every `NET_POLLS` reads of the process table and the previous
/// answer is carried forward in between.
const NET_POLLS: u32 = 8;

/// The store-usage query is the same kind of round trip, so it is throttled the
/// same way and phased against the lease query: the two never share a tick.
const DISK_POLLS: u32 = 8;
const DISK_PHASE: u32 = 4;

#[derive(Clone, Copy)]
pub struct Rail {
    pub sample: Sample,
    pub spark: SparkRing,
    ticks: u32,
    net_polls: u32,
    disk_polls: u32,
    warm: bool,
}

impl Rail {
    pub const fn new() -> Self {
        Rail {
            sample: Sample::EMPTY,
            spark: SparkRing::new(),
            ticks: 0,
            net_polls: 0,
            disk_polls: DISK_PHASE,
            warm: false,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.ticks += 1;
        if self.ticks < POLL_TICKS {
            return false;
        }
        self.ticks = 0;
        let mut next = poll(&self.sample);
        next.net = if self.net_polls == 0 { query() } else { self.sample.net };
        self.net_polls = (self.net_polls + 1) % NET_POLLS;
        next.disk = if self.disk_polls == 0 { disk_usage() } else { self.sample.disk };
        self.disk_polls = (self.disk_polls + 1) % DISK_POLLS;
        if self.warm {
            self.spark.push(next.cpu_pct.min(100) as u8);
        }
        self.warm = true;
        self.sample = next;
        true
    }
}
