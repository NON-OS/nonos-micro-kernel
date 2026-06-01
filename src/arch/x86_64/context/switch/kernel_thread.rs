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

use alloc::sync::Arc;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::arch::x86_64::gdt;
use crate::memory::paging::manager::api::switch_to_process_address_space;
use crate::process::core::{ProcessControlBlock, ProcessState, CURRENT_PID};
use crate::process::nonos_core::{
    has_saved_fpu_state, init_fpu, restore_fpu_state, INTERRUPT_SAVED_CONTEXTS,
};
use crate::process::scheduler::preemption::{CURRENT_TIME_SLICE, DEFAULT_TIME_SLICE};
use crate::smp::percpu;

static RESUME_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn is_traced(pid: u32) -> bool {
    matches!(pid, 7 | 8 | 0x1b | 0x1c | 0x26 | 0x27)
}

fn trace(label: &[u8], pid: u32) {
    if !is_traced(pid) || RESUME_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 32 {
        return;
    }
    crate::sys::serial::trace(b"[KRESUME] ");
    crate::sys::serial::traceln(label);
}

fn trace_ctx(ctx: &crate::sched::Context, pid: u32) {
    if !matches!(pid, 0x26 | 0x27) || RESUME_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 32 {
        return;
    }
    crate::sys::serial::trace(b"[KRESUME] rip=");
    crate::arch::x86_64::diag::print_hex_u64(ctx.rip);
    crate::sys::serial::trace(b" rsp=");
    crate::arch::x86_64::diag::print_hex_u64(ctx.rsp);
    crate::sys::serial::traceln(b"");
}

fn restore_syscall_user_rsp(pcb: &Arc<ProcessControlBlock>) {
    let rsp = pcb.syscall_user_rsp.load(Ordering::Relaxed);
    if rsp == 0 {
        return;
    }
    unsafe {
        core::arch::asm!(
            "mov gs:0x28, {0}",
            in(reg) rsp,
            options(nomem, nostack, preserves_flags),
        );
    }
}

// CPL=0 resume path. Used when the PCB has no pending user-entry and
// no saved user context — typically a kernel thread whose CpuContext
// was parked in INTERRUPT_SAVED_CONTEXTS by the preempt/yield path.
// Returns control to that context via CpuContext::restore.
pub(super) fn resume_kernel_thread(pcb: &Arc<ProcessControlBlock>, pid: u32) {
    trace(b"enter", pid);
    let ctx = match INTERRUPT_SAVED_CONTEXTS.write().remove(&pid) {
        Some(c) => c,
        None => {
            trace(b"missing ctx", pid);
            *pcb.state.lock() = ProcessState::Ready;
            return;
        }
    };

    let kstack = pcb.kernel_stack_top.load(Ordering::Acquire);
    if kstack == 0 {
        trace(b"missing kstack", pid);
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return;
    }

    let cpu = percpu::current().cpu_id;
    unsafe {
        if gdt::set_kernel_stack(cpu, kstack).is_err() {
            trace(b"set kstack failed", pid);
            *pcb.state.lock() = ProcessState::Terminated(-1);
            return;
        }
    }
    percpu::set_kernel_stack(kstack);

    let has_own_addr_space = pcb.cr3.load(Ordering::Relaxed) != 0;
    *pcb.state.lock() = ProcessState::Running;
    CURRENT_PID.store(pid, Ordering::SeqCst);
    CURRENT_TIME_SLICE.store(DEFAULT_TIME_SLICE, Ordering::SeqCst);

    if has_own_addr_space {
        let _ = switch_to_process_address_space(pid);
    }

    if has_saved_fpu_state(pid) {
        restore_fpu_state(pid);
    } else {
        init_fpu();
    }

    restore_syscall_user_rsp(pcb);
    trace_ctx(&ctx, pid);
    trace(b"restore", pid);
    ctx.restore()
}
