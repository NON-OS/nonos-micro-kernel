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

use crate::syscall::{call_raw, N_MK_PROC_STAT};

// Inline process name length; must match the kernel's PROC_NAME_LEN.
pub const PROC_NAME_LEN: usize = 24;

// Mirrors the kernel ProcStatEntry wire layout exactly (see
// src/syscall/microkernel/procstat.rs). state: 0 new, 1 ready, 2 running,
// 3 sleeping, 4 stopped, 5 zombie, 6 terminated.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcStatEntry {
    pub pid: u32,
    pub state: u8,
    pub name_len: u8,
    pub _pad: [u8; 2],
    pub run_ticks: u64,
    pub caps: u64,
    pub mem_kb: u64,
    pub uptime_ms: u64,
    pub name: [u8; PROC_NAME_LEN],
}

impl Default for ProcStatEntry {
    fn default() -> Self {
        ProcStatEntry {
            pid: 0,
            state: 0,
            name_len: 0,
            _pad: [0; 2],
            run_ticks: 0,
            caps: 0,
            mem_kb: 0,
            uptime_ms: 0,
            name: [0; PROC_NAME_LEN],
        }
    }
}

impl ProcStatEntry {
    pub fn name_str(&self) -> &str {
        let n = (self.name_len as usize).min(PROC_NAME_LEN);
        core::str::from_utf8(&self.name[..n]).unwrap_or("")
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ProcStatHeader {
    pub total_ticks: u64,
    pub count: u32,
    pub _pad: u32,
    pub mem_total_kb: u64,
    // 1/5/15-minute load averages in Q11 fixed point: 2048 reads as 1.00.
    pub load_avg_fixed: [u64; 3],
}

pub extern "C" fn mk_proc_stat(buf: *mut u8, max_entries: u32) -> i64 {
    call_raw(N_MK_PROC_STAT, [buf as u64, max_entries as u64, 0, 0, 0, 0])
}
