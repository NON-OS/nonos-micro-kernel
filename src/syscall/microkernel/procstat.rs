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

//! `MkProcStat` copies a `ProcStatHeader` plus one `ProcStatEntry` per
//! live pid; a NULL buffer or zero `max_entries` probes the pid count.

use core::mem::size_of;

use super::errnos::ERRNO_FAULT;
use crate::process::scheduler::preemption::proc_ticks;
use crate::usercopy::{validate_user_write, write_user_value};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcStatEntry {
    pub pid: u32,
    pub state: u8,
    pub _pad: [u8; 3],
    pub run_ticks: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcStatHeader {
    pub total_ticks: u64,
    pub count: u32,
    pub _pad: u32,
}

pub fn sys_proc_stat(buf_ptr: u64, max_entries: u64) -> i64 {
    let pids = crate::process::list_all_pids();
    if max_entries == 0 || buf_ptr == 0 {
        return pids.len() as i64;
    }
    let to_write = core::cmp::min(max_entries as usize, pids.len());
    let bytes = size_of::<ProcStatHeader>() + to_write * size_of::<ProcStatEntry>();
    if validate_user_write(buf_ptr, bytes).is_err() {
        return ERRNO_FAULT;
    }
    let header = ProcStatHeader {
        total_ticks: crate::interrupts::timer::state::get_ticks(),
        count: to_write as u32,
        _pad: 0,
    };
    if write_user_value(buf_ptr, &header).is_err() {
        return ERRNO_FAULT;
    }
    let mut dst = buf_ptr + size_of::<ProcStatHeader>() as u64;
    for pid in pids.iter().take(to_write) {
        let entry = ProcStatEntry {
            pid: *pid,
            state: 0,
            _pad: [0; 3],
            run_ticks: proc_ticks::ticks_for(*pid),
        };
        if write_user_value(dst, &entry).is_err() {
            return ERRNO_FAULT;
        }
        dst += size_of::<ProcStatEntry>() as u64;
    }
    to_write as i64
}
