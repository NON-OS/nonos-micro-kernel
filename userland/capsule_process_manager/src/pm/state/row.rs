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

//! One process as the kernel reports it, plus the rates derived from two
//! consecutive reports.

use nonos_libc::PROC_NAME_LEN;

use super::row_from::Rates;

#[derive(Clone)]
pub struct Row {
    pub pid: u32,
    pub ppid: u32,
    pub name: [u8; PROC_NAME_LEN],
    pub name_len: u8,
    pub state: u8,
    pub priority: u8,
    pub caps: u64,
    pub mem_kb: u64,
    pub mapped_kb: u64,
    pub vma_count: u32,
    pub cpu_pct: u8,
    pub user_pct: u8,
    pub uptime_ms: u64,
    pub syscalls: u64,
    pub ipc_tx: u64,
    pub ipc_rx: u64,
    pub faults: u64,
    pub switches: u64,
    pub sysc_ps: u32,
    pub ipc_ps: u32,
    pub fault_ps: u32,
}

impl Row {
    pub fn name(&self) -> &[u8] {
        &self.name[..(self.name_len as usize).min(PROC_NAME_LEN)]
    }
}

impl Row {
    pub fn rates(&self) -> Rates {
        Rates {
            cpu_pct: self.cpu_pct,
            user_pct: self.user_pct,
            sysc_ps: self.sysc_ps,
            ipc_ps: self.ipc_ps,
            fault_ps: self.fault_ps,
        }
    }
}
