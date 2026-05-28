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

use super::super::dispatch::add_to_run_queue;
use super::super::selection::{select_next_process, switch_to_process};
use super::save_syscall_user_rsp;
use super::state::CURRENT_TIME_SLICE;
use core::sync::atomic::{AtomicU32, Ordering};

static YIELD_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn is_traced(pid: u32) -> bool {
    matches!(pid, 7 | 8 | 0x1b | 0x1c | 0x26 | 0x27)
}

fn trace(label: &[u8], pid: u32) {
    if !is_traced(pid) || YIELD_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 40 {
        return;
    }
    crate::sys::serial::print(b"[YIELD] ");
    crate::sys::serial::println(label);
}

fn trace_return_slot(pid: u32) {
    if !matches!(pid, 0x26 | 0x27) || YIELD_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 40 {
        return;
    }
    let rsp: u64;
    let rbp: u64;
    unsafe {
        core::arch::asm!(
            "mov {0}, rsp",
            "mov {1}, rbp",
            out(reg) rsp,
            out(reg) rbp,
            options(nomem, nostack, preserves_flags),
        );
    }
    let ret = unsafe { *((rbp + 8) as *const u64) };
    crate::sys::serial::print(b"[YIELD] rsp=");
    crate::arch::x86_64::diag::print_hex_u64(rsp);
    crate::sys::serial::print(b" rbp=");
    crate::arch::x86_64::diag::print_hex_u64(rbp);
    crate::sys::serial::print(b" ret=");
    crate::arch::x86_64::diag::print_hex_u64(ret);
    crate::sys::serial::println(b"");
}

/// Voluntary-yield body. Runs with interrupts already disabled by the
/// caller. The contract backend dispatches `SwitchIntent::Yield` here.
#[inline(never)]
pub(crate) fn perform_yield_inline() {
    use crate::process::nonos_core::{current_pid, ProcessState, PROCESS_TABLE};

    let Some(pid) = current_pid() else { return };
    trace(b"enter", pid);

    let mut ctx: crate::sched::Context = unsafe { core::mem::zeroed() };
    crate::sched::Context::clear_restored_flag();
    unsafe { crate::sched::Context::save_to(&mut ctx as *mut crate::sched::Context) };
    if crate::sched::Context::was_just_restored() {
        trace_return_slot(pid);
        trace(b"restored", pid);
        return;
    }

    save_syscall_user_rsp(pid);
    crate::process::nonos_core::save_interrupt_context(pid, ctx);
    crate::process::nonos_core::save_fpu_state(pid);

    let runnable = if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Running) {
            *state = ProcessState::Ready;
        }
        matches!(*state, ProcessState::Ready)
    } else {
        false
    };

    if runnable {
        add_to_run_queue(pid);
    }
    CURRENT_TIME_SLICE.store(0, Ordering::Relaxed);

    if let Some(next) = select_next_process() {
        if next != pid {
            trace(b"switch away", pid);
            switch_to_process(next);
        } else if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
            let mut state = pcb.state.lock();
            if matches!(*state, ProcessState::Ready) {
                *state = ProcessState::Running;
            }
        }
    }
}
