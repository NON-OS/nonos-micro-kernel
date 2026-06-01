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

extern crate alloc;
use super::pcb::ProcessControlBlock;
use super::table::{ProcessTable, CURRENT_PID, PROCESS_TABLE};
use super::types::{Pid, ProcessState};
use alloc::sync::Arc;
use core::sync::atomic::Ordering;

pub fn init_process_management() {
    crate::process::core::init::init_system_processes();
    crate::log::info!("[PROCESS] Process management initialized");
}

#[inline]
pub fn get_process_table() -> &'static ProcessTable {
    &PROCESS_TABLE
}
#[inline]
pub fn current_pid() -> Option<Pid> {
    match CURRENT_PID.load(Ordering::SeqCst) {
        0 => None,
        v => Some(v),
    }
}
#[inline]
pub fn current_process() -> Option<Arc<ProcessControlBlock>> {
    PROCESS_TABLE.find_by_pid(current_pid()?)
}
#[inline]
pub fn is_process_active(name: &str) -> bool {
    PROCESS_TABLE.is_active_name(name)
}
#[inline]
pub fn is_process_active_by_id(pid: u64) -> bool {
    PROCESS_TABLE.is_active_pid(pid)
}

#[inline]
pub fn context_switch(to: Pid) -> Result<(), &'static str> {
    if PROCESS_TABLE.find_by_pid(to).is_none() {
        return Err("not found");
    }
    CURRENT_PID.store(to, Ordering::SeqCst);
    Ok(())
}

/// Clear `CURRENT_PID` if the calling CPU still has `pid` recorded
/// as current. Called from `process::exit::teardown` after the PCB
/// has been dropped, so the next scheduler tick or trap handler does
/// not observe a stale dead pid before it dispatches a fresh one.
#[inline]
pub fn clear_current_if(pid: Pid) {
    if CURRENT_PID.load(Ordering::Acquire) == pid {
        CURRENT_PID.store(0, Ordering::Release);
    }
}

#[derive(Default)]
pub struct ProcessManagementStats {
    pub total: u64,
    pub running: u64,
    pub sleeping: u64,
    pub stopped: u64,
}

pub fn get_process_stats() -> ProcessManagementStats {
    let mut s = ProcessManagementStats::default();
    let list = get_process_table().get_all_processes();
    s.total = list.len() as u64;
    for p in list {
        match *p.state.lock() {
            ProcessState::Running => s.running += 1,
            ProcessState::Sleeping => s.sleeping += 1,
            ProcessState::Stopped => s.stopped += 1,
            _ => {}
        }
    }
    s
}

pub mod syscalls {
    pub fn sys_exit(code: i32) -> ! {
        crate::process::exit::exit_and_yield(code, false)
    }
}
