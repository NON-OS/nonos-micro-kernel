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
use core::sync::atomic::Ordering;

use crate::arch::x86_64::gdt;
use crate::memory::paging::manager::api::switch_to_process_address_space;
use crate::process::core::{ProcessControlBlock, ProcessState, CURRENT_PID};
use crate::process::nonos_core::{has_saved_fpu_state, init_fpu, restore_fpu_state};
use crate::process::scheduler::preemption::{CURRENT_TIME_SLICE, DEFAULT_TIME_SLICE};
use crate::process::userspace::transitions::restore_user_context_iretq;
use crate::smp::percpu;

use super::{build_initial_switch_frame, resume_user_trampoline};

// Preempt-resume path: a CPL=3 capsule was trapped/IRQ'd back into the
// kernel by the trap trampoline, which captured a full UserContext on
// the PCB. Take() consumes the snapshot so a subsequent resume sees
// the next captured frame, never a stale one.
pub(super) fn try_resume(pcb: &Arc<ProcessControlBlock>, pid: u32) -> bool {
    let saved = match pcb.saved_user_context.lock().take() {
        Some(s) => s,
        None => return false,
    };

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

    let switch_rsp = build_initial_switch_frame(kstack, resume_user_trampoline as u64);
    pcb.kernel_rsp.store(switch_rsp, Ordering::Release);
    #[cfg(feature = "nonos-cpuswitch-selftest")]
    {
        crate::sys::serial::print(b"[CPUSWITCH] preempt kernel_rsp set pid=");
        crate::arch::x86_64::diag::print_hex_u64(pid as u64);
        crate::sys::serial::println(b"");
    }

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

    // SAFETY: `saved` fully populated; asm consumes via rdi and iretqs.
    unsafe { restore_user_context_iretq(&saved as *const _) }
}
