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

//! A row built from one kernel entry and the rates the refresh derived.

use nonos_libc::ProcStatEntry;

use super::row::Row;

/// Cpu share, its user part, and the per-second rates, as the refresh
/// computed them from two consecutive entries.
#[derive(Clone, Copy, Default)]
pub struct Rates {
    pub cpu_pct: u8,
    pub user_pct: u8,
    pub sysc_ps: u32,
    pub ipc_ps: u32,
    pub fault_ps: u32,
}

impl Row {
    pub fn from_entry(e: &ProcStatEntry, r: Rates) -> Row {
        Row {
            pid: e.pid,
            ppid: e.ppid,
            name: e.name,
            name_len: e.name_len,
            state: e.state,
            priority: e.priority,
            caps: e.caps,
            mem_kb: e.mem_kb,
            mapped_kb: e.mapped_kb,
            vma_count: e.vma_count,
            cpu_pct: r.cpu_pct,
            user_pct: r.user_pct,
            uptime_ms: e.uptime_ms,
            syscalls: e.syscalls,
            ipc_tx: e.ipc_tx,
            ipc_rx: e.ipc_rx,
            faults: e.faults,
            switches: e.switches,
            sysc_ps: r.sysc_ps,
            ipc_ps: r.ipc_ps,
            fault_ps: r.fault_ps,
        }
    }
}
