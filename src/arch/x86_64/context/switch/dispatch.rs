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

use crate::process::core::PROCESS_TABLE;
use crate::process::nonos_core::INTERRUPT_SAVED_CONTEXTS;
use core::sync::atomic::{AtomicU32, Ordering};

use super::first_entry::try_first_entry;
use super::kernel_thread::resume_kernel_thread;
use super::resume::try_resume;

static DISPATCH_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace(label: &[u8], pid: u32) {
    if !matches!(pid, 7 | 8 | 9 | 0x1b | 0x1c | 0x26 | 0x27)
        || DISPATCH_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 64
    {
        return;
    }
    crate::sys::serial::print(b"[DISPATCH] pid=");
    crate::sys::serial::print_hex(pid as u64);
    crate::sys::serial::print(b" ");
    crate::sys::serial::println(label);
}

pub(crate) fn switch_to_user_pcb_x86_64(pid: u32) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };

    if try_first_entry(&pcb, pid) {
        return;
    }
    // A pid blocked inside a syscall can have both a stale user-mode
    // trap snapshot and a live kernel resume context. The kernel
    // context must win; resuming the stale user frame re-enters at an
    // arbitrary old RIP and skips the syscall continuation entirely.
    if INTERRUPT_SAVED_CONTEXTS.read().contains_key(&pid) {
        trace(b"kernel ctx", pid);
        resume_kernel_thread(&pcb, pid);
        return;
    }
    if try_resume(&pcb, pid) {
        trace(b"user ctx", pid);
        return;
    }
    trace(b"fallback kernel", pid);
    resume_kernel_thread(&pcb, pid);
}
