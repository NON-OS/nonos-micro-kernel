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
use crate::process::nonos_core::{has_saved_fpu_state, init_fpu, restore_fpu_state};
use crate::process::scheduler::preemption::{CURRENT_TIME_SLICE, DEFAULT_TIME_SLICE};
use crate::process::userspace::transitions::restore_user_context_iretq;
use crate::smp::percpu;

static USER_RESUME_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace(label: &[u8], pid: u32) {
    if !matches!(pid, 7 | 8 | 0x1c | 0x27)
        || USER_RESUME_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 24
    {
        return;
    }
    crate::sys::serial::print(b"[URESUME] ");
    crate::sys::serial::println(label);
}

// Preempt-resume path: a CPL=3 capsule was trapped/IRQ'd back into the
// kernel by the trap trampoline, which captured a full UserContext on
// the PCB. Take() consumes the snapshot so a subsequent resume sees
// the next captured frame, never a stale one.
pub(super) fn try_resume(pcb: &Arc<ProcessControlBlock>, pid: u32) -> bool {
    let saved = match pcb.saved_user_context.lock().take() {
        Some(s) => s,
        None => return false,
    };
    trace(b"enter", pid);

    let kstack = pcb.kernel_stack_top.load(Ordering::Acquire);
    if kstack == 0 {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }

    let cpu = percpu::current().cpu_id;
    // SAFETY: cpu bounded by MAX_CPUS; set_kernel_stack validates.
    unsafe {
        if gdt::set_kernel_stack(cpu, kstack).is_err() {
            *pcb.state.lock() = ProcessState::Terminated(-1);
            return true;
        }
    }
    percpu::set_kernel_stack(kstack);

    if pcb.cr3.load(Ordering::Relaxed) != 0
        && switch_to_process_address_space(pid).is_err()
    {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }

    *pcb.state.lock() = ProcessState::Running;
    CURRENT_PID.store(pid, Ordering::SeqCst);
    CURRENT_TIME_SLICE.store(DEFAULT_TIME_SLICE, Ordering::SeqCst);

    if has_saved_fpu_state(pid) {
        restore_fpu_state(pid);
    } else {
        init_fpu();
    }

    trace(b"iretq", pid);
    // SAFETY: `saved` fully populated; asm consumes via rdi and iretqs.
    unsafe { restore_user_context_iretq(&saved as *const _) }
}
