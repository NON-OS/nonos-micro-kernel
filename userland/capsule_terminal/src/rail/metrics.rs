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

use super::disk::Disk;
use super::mem::Mem;
use super::net::Net;
use super::value::Metric;

pub const MAX_PROCS: usize = 64;

#[derive(Clone, Copy)]
pub struct Proc {
    pub pid: u32,
    pub cpu_pct: u32,
    pub mem_kb: u64,
    pub run_ticks: u64,
}

impl Proc {
    pub const EMPTY: Proc = Proc { pid: 0, cpu_pct: 0, mem_kb: 0, run_ticks: 0 };
}

#[derive(Clone, Copy)]
pub struct Sample {
    pub total_ticks: u64,
    pub procs: [Proc; MAX_PROCS],
    pub n: usize,
    pub mem_total_kb: u64,
    pub cpu_pct: u32,
    pub uptime_ms: u64,
    pub mem: Mem,
    pub net: Net,
    pub disk: Disk,
    /// The scheduler keeps no runnable-queue average, so a load figure would be
    /// invented rather than measured.
    pub load_avg: Metric<u32>,
}

impl Sample {
    pub const EMPTY: Sample = Sample {
        total_ticks: 0,
        procs: [Proc::EMPTY; MAX_PROCS],
        n: 0,
        mem_total_kb: 0,
        cpu_pct: 0,
        uptime_ms: 0,
        mem: Mem::UNKNOWN,
        net: Net::DOWN,
        disk: Disk::UNSUPPORTED,
        load_avg: Metric::Unsupported,
    };

    pub fn live(&self) -> &[Proc] {
        &self.procs[..self.n.min(MAX_PROCS)]
    }
}
