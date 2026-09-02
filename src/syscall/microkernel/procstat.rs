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
use core::sync::atomic::Ordering;

use super::errnos::ERRNO_FAULT;
use crate::process::core::types::ProcessState;
use crate::process::scheduler::preemption::proc_ticks;
use crate::usercopy::{validate_user_write, write_user_value};

// Name bytes carried inline so a monitor needs no extra lookup per pid.
pub const PROC_NAME_LEN: usize = 24;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcStatEntry {
    pub pid: u32,
    // 0 new, 1 ready, 2 running, 3 sleeping, 4 stopped, 5 zombie, 6 terminated.
    pub state: u8,
    pub name_len: u8,
    pub _pad: [u8; 2],
    pub run_ticks: u64,
    pub caps: u64,
    pub mem_kb: u64,
    // Milliseconds this process has been alive, computed in the kernel against
    // one monotonic clock so the reader never has to reconcile two time bases.
    pub uptime_ms: u64,
    pub name: [u8; PROC_NAME_LEN],
}

fn state_code(s: &ProcessState) -> u8 {
    match s {
        ProcessState::New => 0,
        ProcessState::Ready => 1,
        ProcessState::Running => 2,
        ProcessState::Sleeping => 3,
        ProcessState::Stopped => 4,
        ProcessState::Zombie(_) => 5,
        ProcessState::Terminated(_) => 6,
    }
}

// Fill an entry with the live truth for `pid`: name, state, granted caps and
// resident memory, all straight from the process control block.
fn entry_for(pid: u32, now_ms: u64) -> ProcStatEntry {
    let mut e = ProcStatEntry {
        pid,
        state: 0,
        name_len: 0,
        _pad: [0; 2],
        run_ticks: proc_ticks::ticks_for(pid),
        caps: 0,
        mem_kb: 0,
        uptime_ms: 0,
        name: [0; PROC_NAME_LEN],
    };
    crate::process::with_process(pid, |pcb| {
        e.state = state_code(&pcb.state.lock());
        e.caps = pcb.caps_bits.load(Ordering::Relaxed);
        e.mem_kb = pcb.memory.lock().resident_pages.load(Ordering::Relaxed) as u64 * 4;
        // Both `now_ms` and start use the same monotonic millisecond clock, so
        // the difference is a real elapsed time, never a wall-clock epoch.
        let start_ms = pcb.start_time_ms.load(Ordering::Relaxed);
        e.uptime_ms = now_ms.saturating_sub(start_ms);
        let name = pcb.name.lock();
        let bytes = name.as_bytes();
        let n = bytes.len().min(PROC_NAME_LEN);
        e.name[..n].copy_from_slice(&bytes[..n]);
        e.name_len = n as u8;
    });
    e
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcStatHeader {
    pub total_ticks: u64,
    pub count: u32,
    pub _pad: u32,
    // Physical memory the boot memory map handed the allocator, so a monitor
    // can size a usage bar without a second syscall.
    pub mem_total_kb: u64,
    // 1/5/15-minute load averages in Q11 fixed point: 2048 reads as 1.00.
    // Integer-only so the syscall ABI never carries a float.
    pub load_avg_fixed: [u64; 3],
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
        mem_total_kb: crate::memory::phys::allocator::phys_total_memory() / 1024,
        load_avg_fixed: crate::fs::procfs::load_averages_fixed(),
    };
    if write_user_value(buf_ptr, &header).is_err() {
        return ERRNO_FAULT;
    }
    let now_ms = crate::time::timestamp_millis();
    let mut dst = buf_ptr + size_of::<ProcStatHeader>() as u64;
    for pid in pids.iter().take(to_write) {
        let entry = entry_for(*pid, now_ms);
        if write_user_value(dst, &entry).is_err() {
            return ERRNO_FAULT;
        }
        dst += size_of::<ProcStatEntry>() as u64;
    }
    to_write as i64
}
