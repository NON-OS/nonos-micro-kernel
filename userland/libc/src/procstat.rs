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

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ProcStatEntry {
    pub pid: u32,
    pub state: u8,
    pub _pad: [u8; 3],
    pub run_ticks: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ProcStatHeader {
    pub total_ticks: u64,
    pub count: u32,
    pub _pad: u32,
}

pub extern "C" fn mk_proc_stat(buf: *mut u8, max_entries: u32) -> i64 {
    call_raw(N_MK_PROC_STAT, [buf as u64, max_entries as u64, 0, 0, 0, 0])
}
