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
use crate::process::scheduler::preemption::{set_time_slice, DEFAULT_TIME_SLICE};
use crate::process::userspace::transitions::restore_user_context_iretq;
use crate::smp::percpu;

use super::validate_resume::sanitize_user_context;

pub(super) fn try_resume(pcb: &Arc<ProcessControlBlock>, pid: u32) -> bool {
    let mut saved = match pcb.saved_user_context.lock().take() {
        Some(s) => s,
        None => return false,
    };
    if !sanitize_user_context(&mut saved) {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }

    let kstack = pcb.kernel_stack_top.load(Ordering::Acquire);
    if kstack == 0 {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }

    let cpu = percpu::current().cpu_id;
    unsafe {
        if gdt::set_kernel_stack(cpu, kstack).is_err() {
            *pcb.state.lock() = ProcessState::Terminated(-1);
            return true;
        }
    }
    percpu::set_kernel_stack(kstack);

    let cr3v = pcb.cr3.load(Ordering::Relaxed);
    if cr3v != 0 && switch_to_process_address_space(pid).is_err() {
        crate::memory::paging::tlb::set_cr3(crate::memory::PhysAddr::new(cr3v));
    }

    *pcb.state.lock() = ProcessState::Running;
    CURRENT_PID.store(pid, Ordering::SeqCst);
    set_time_slice(DEFAULT_TIME_SLICE);

    if has_saved_fpu_state(pid) {
        restore_fpu_state(pid);
    } else {
        init_fpu();
    }

    unsafe { restore_user_context_iretq(&saved as *const _) }
}
