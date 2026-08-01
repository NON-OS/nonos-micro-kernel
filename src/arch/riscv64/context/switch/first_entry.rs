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

use crate::arch::riscv64::context::enter::enter_user;
use crate::arch::riscv64::fpu;
use crate::process::core::{ProcessControlBlock, ProcessState, CURRENT_PID};
use crate::process::scheduler::preemption::{set_time_slice, DEFAULT_TIME_SLICE};

use super::address_space::swap_address_space;

pub(super) fn try_first_entry(pcb: &Arc<ProcessControlBlock>, pid: u32) -> bool {
    let mut entry = match pcb.pending_user_entry.lock().take() {
        Some(e) => e,
        None => return false,
    };

    let kstack = pcb.kernel_stack_top.load(Ordering::Acquire);
    if kstack == 0 {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }
    if entry.kernel_sp == 0 {
        entry.kernel_sp = kstack;
    } else if entry.kernel_sp != kstack {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }

    if swap_address_space(pcb).is_err() {
        *pcb.state.lock() = ProcessState::Terminated(-1);
        return true;
    }

    *pcb.state.lock() = ProcessState::Running;
    CURRENT_PID.store(pid, Ordering::SeqCst);
    set_time_slice(DEFAULT_TIME_SLICE);

    fpu::prepare_incoming();

    if unsafe { enter_user(&entry) }.is_err() {
        *pcb.state.lock() = ProcessState::Terminated(-1);
    }
    true
}
