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

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};
use crate::process::core::{create_process, Priority, ProcessState};
use crate::process::current_pid;
use core::sync::atomic::{AtomicU32, Ordering};

const MAX_NAME_LEN: usize = 256;
static EXIT_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace_exit(label: &[u8], pid: u32) {
    if !matches!(pid, 7 | 8 | 0x1b | 0x1c | 0x27)
        || EXIT_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 24
    {
        return;
    }
    crate::sys::serial::print(b"[EXIT] ");
    crate::sys::serial::println(label);
}

pub fn sys_spawn(name_ptr: u64, name_len: usize) -> i64 {
    if name_len == 0 || name_len > MAX_NAME_LEN {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_read(name_ptr, name_len).is_err() {
        return ERRNO_FAULT;
    }
    let mut name_bytes = alloc::vec![0u8; name_len];
    if crate::usercopy::copy_from_user(name_ptr, &mut name_bytes).is_err() {
        return ERRNO_FAULT;
    }
    let name = match core::str::from_utf8(&name_bytes) {
        Ok(s) => s,
        Err(_) => return ERRNO_FAULT,
    };
    match create_process(name, ProcessState::Ready, Priority::Normal) {
        Ok(pid) => pid as i64,
        Err(_) => ERRNO_NOMEM,
    }
}

pub fn sys_exit(code: i32) -> i64 {
    let Some(pid) = current_pid() else {
        return ERRNO_INVAL;
    };
    let _ = code;
    trace_exit(b"enter", pid);
    crate::process::exit::exit_and_yield(code, false)
}

pub fn sys_pid_alive(pid: u32) -> i64 {
    if pid == 0 {
        return 0;
    }
    if crate::process::get_process_table().find_by_pid(pid).is_some() {
        1
    } else {
        0
    }
}

pub fn sys_yield() -> i64 {
    crate::sched::yield_now();
    0
}

// Deschedule the caller for `ms` milliseconds. Unlike yield, which leaves
// the task runnable and so keeps a busy backoff loop pinning the CPU, this
// parks the task until its wake deadline so the core can idle. Service
// capsules that poll for a dependency to come up use this instead of a tight
// yield loop, which is what used to peg the host core and starve the desktop.
pub fn sys_sleep_ms(ms: u64) -> i64 {
    let Some(pid) = current_pid() else {
        return ERRNO_PERM;
    };
    if ms == 0 {
        crate::sched::yield_now();
        return 0;
    }
    let deadline = crate::time::timestamp_millis().saturating_add(ms);
    while crate::time::timestamp_millis() < deadline {
        crate::sched::sleep_until(pid, deadline);
        crate::sched::yield_now();
    }
    0
}

pub fn sys_getpid() -> i64 {
    match current_pid() {
        Some(pid) => pid as i64,
        None => ERRNO_PERM,
    }
}

pub fn sys_args(buf: u64, len: usize) -> i64 {
    let Some(pid) = current_pid() else {
        return ERRNO_PERM;
    };
    let Some(pcb) = crate::process::get_process_table().find_by_pid(pid) else {
        return ERRNO_INVAL;
    };
    let mut blob: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
    for (i, arg) in pcb.argv.lock().iter().enumerate() {
        if i > 0 {
            blob.push(0);
        }
        blob.extend_from_slice(arg.as_bytes());
    }
    if buf == 0 || len < blob.len() {
        return blob.len() as i64;
    }
    if crate::usercopy::copy_to_user(buf, &blob).is_err() {
        return ERRNO_FAULT;
    }
    blob.len() as i64
}

pub fn sys_thread_spawn(entry: u64, stack: u64) -> i64 {
    if entry == 0 || stack == 0 {
        return ERRNO_INVAL;
    }
    match crate::process::core::spawn_thread(entry, stack) {
        Ok(tid) => tid as i64,
        Err(_) => ERRNO_NOMEM,
    }
}

pub fn sys_set_tls(base: u64) -> i64 {
    // Zero clears the base; anything else must point at readable user
    // memory (userspace keeps the TLS table's self-pointer at offset 0,
    // so the first eight bytes are the smallest meaningful window).
    if base != 0 && crate::usercopy::validate_user_read(base, 8).is_err() {
        return ERRNO_FAULT;
    }
    let Some(pid) = current_pid() else {
        return ERRNO_PERM;
    };
    let Some(pcb) = crate::process::get_process_table().find_by_pid(pid) else {
        return ERRNO_INVAL;
    };
    if !crate::arch::context::set_user_tls(base) {
        // The arch cannot carry a user TLS register across its return
        // path yet; refuse rather than let userspace believe TLS works.
        return ERRNO_INVAL;
    }
    pcb.set_tls_base(base);
    0
}
