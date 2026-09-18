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

//! The machine as a whole, from the header of one `MkProcStat` read and
//! its difference from the previous one. Every rate here is a count the
//! kernel kept divided by the milliseconds between two reads; nothing is
//! estimated.

use nonos_libc::ProcStatHeader;

/// One process's cumulative counters at the previous read, keyed by pid.
#[derive(Clone, Copy)]
pub struct Prev {
    pub pid: u32,
    pub ticks: u64,
    pub user_ticks: u64,
    pub syscalls: u64,
    pub ipc: u64,
    pub faults: u64,
}

#[derive(Clone, Copy, Default)]
pub struct System {
    /// Busy, and the split of busy into user and kernel; idle is the rest.
    pub busy_pct: u8,
    pub user_pct: u8,
    pub kernel_pct: u8,
    pub idle_pct: u8,
    pub cpus: u32,
    pub mem_total_kb: u64,
    pub mem_free_kb: u64,
    pub largest_free_kb: u64,
    pub heap_used_kb: u64,
    pub heap_total_kb: u64,
    pub heap_peak_kb: u64,
    pub heap_allocs: u64,
    pub uptime_ms: u64,
    /// 1, 5 and 15 minute load in Q11 fixed point.
    pub load: [u64; 3],
    pub ctx_ps: u32,
    pub sysc_ps: u32,
    pub ipc_ps: u32,
    pub irq_ps: u32,
    pub fault_ps: u32,
    pub(super) prev: Option<ProcStatHeader>,
}

impl System {
    pub fn mem_used_kb(&self) -> u64 {
        self.mem_total_kb.saturating_sub(self.mem_free_kb)
    }
}
