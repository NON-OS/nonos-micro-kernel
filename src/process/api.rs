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

use super::core::{
    current_pid, get_process_table, suspend_process, ProcessControlBlock, ProcessState,
};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

pub fn get_current_pty() -> Option<u32> {
    let pid = current_pid()?;
    let proc = get_process(pid)?;
    let tty = proc.tty_nr.load(Ordering::Acquire);
    if tty == 0 {
        None
    } else {
        Some(tty)
    }
}

pub fn list_all_pids() -> Vec<u32> {
    get_process_table().get_all_processes().iter().map(|p| p.pid).collect()
}

pub fn last_pid() -> u32 {
    current_pid().unwrap_or(1)
}
pub fn current_tid() -> u32 {
    current_pid().unwrap_or(1)
}

pub fn get_process(pid: u32) -> Option<Arc<ProcessControlBlock>> {
    get_process_table().find_by_pid(pid)
}

/// Run `f` against the PCB for `pid`, returning its result, or `None` if the
/// pid is unknown. Field-level mutability is reached through the PCB's interior
/// locks; this helper just resolves the pid and keeps the Arc alive for the
/// duration of the closure.
pub fn with_process<R>(pid: u32, f: impl FnOnce(&Arc<ProcessControlBlock>) -> R) -> Option<R> {
    get_process(pid).map(|pcb| f(&pcb))
}

/// Mutable counterpart to [`with_process`]. The PCB is shared via `Arc`, so
/// "mut" here is a calling convention — actual mutation happens through the
/// per-field `Mutex`/atomics inside the PCB.
pub fn with_process_mut<R>(pid: u32, f: impl FnOnce(&Arc<ProcessControlBlock>) -> R) -> Option<R> {
    get_process(pid).map(|pcb| f(&pcb))
}

/// Exit status follows POSIX `128 + signo` for processes killed by signal.
pub fn terminate_current_with_signal(signo: u8) -> ! {
    crate::process::exit::exit_and_yield(signo as i32 + 128, true)
}

pub fn stop_process(pid: u32) -> Result<(), i32> {
    suspend_process(pid).map_err(|_| -3)
}

pub fn resume_process_by_pid(pid: u32) -> Result<(), i32> {
    super::core::resume_process(pid).map_err(|_| -3)
}

pub fn resume_process(pid: u32) -> Result<(), i32> {
    super::core::resume_process(pid).map_err(|_| -3)
}

pub fn current_uid() -> u32 {
    get_process(current_pid().unwrap_or(1)).map(|p| p.creds.lock().uid).unwrap_or(0)
}

pub fn set_cwd(pid: u32, path: &str) -> Result<(), i32> {
    let proc = get_process(pid).ok_or(-3)?;
    let mut cwd = proc.cwd.lock();
    cwd.clear();
    cwd.push_str(path);
    Ok(())
}

pub fn set_comm(pid: u32, name: &str) -> Result<(), i32> {
    get_process(pid).ok_or(-3)?.set_name(name);
    Ok(())
}

pub fn get_uid(pid: u32) -> Option<u32> {
    get_process(pid).map(|p| p.creds.lock().uid)
}

pub fn get_parent_pid(pid: u32) -> Option<u32> {
    get_process(pid).map(|p| p.parent_pid())
}

/// `Some(code)` once `pid` has exited (zombie or already reaped), `None`
/// while it is still running or unknown. Once the process has been reaped,
/// its logged status is consumed on read and returned exactly once.
pub fn exit_status(pid: u32) -> Option<i32> {
    if let Some(pcb) = get_process(pid) {
        return match *pcb.state.lock() {
            ProcessState::Zombie(code) | ProcessState::Terminated(code) => Some(code),
            _ => None,
        };
    }
    crate::process::exit::reap_exit_status(pid)
}

pub fn set_controlling_tty(pid: u32, tty: u32) -> Result<(), i32> {
    get_process(pid).ok_or(-3)?.tty_nr.store(tty, Ordering::Release);
    Ok(())
}

pub fn release_controlling_tty(pid: u32) -> Result<(), i32> {
    let proc = get_process(pid).ok_or(-3)?;
    proc.tty_pgrp.store(-1, Ordering::Release);
    proc.tty_nr.store(0, Ordering::Release);
    Ok(())
}

pub fn get_tty_pgrp(pid: u32) -> Option<i32> {
    get_process(pid).map(|p| p.tty_pgrp.load(Ordering::Acquire))
}

pub fn set_tty_pgrp(pid: u32, pgrp: i32) -> Result<(), i32> {
    get_process(pid).ok_or(-3)?.tty_pgrp.store(pgrp, Ordering::Release);
    Ok(())
}
