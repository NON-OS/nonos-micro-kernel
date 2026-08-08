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

use alloc::collections::BTreeMap;
use core::sync::atomic::Ordering;
use spin::RwLock;

use super::table::{CURRENT_PID, PROCESS_TABLE};
use super::types::{Pid, ProcessState, SuspendedContext};
use super::{context_switch, current_pid};
use crate::process::userspace::types::FpuState;

static SUSPENDED_CONTEXTS: RwLock<BTreeMap<Pid, SuspendedContext>> = RwLock::new(BTreeMap::new());

pub static INTERRUPT_SAVED_CONTEXTS: RwLock<BTreeMap<Pid, crate::sched::Context>> =
    RwLock::new(BTreeMap::new());

pub static INTERRUPT_SAVED_FPU_STATES: RwLock<BTreeMap<Pid, FpuState>> =
    RwLock::new(BTreeMap::new());

pub fn suspend_process(pid: Pid) -> Result<(), &'static str> {
    let pcb = PROCESS_TABLE.find_by_pid(pid).ok_or("Process not found")?;

    let current_state = *pcb.state.lock();
    match current_state {
        ProcessState::Terminated(_) | ProcessState::Zombie(_) => {
            return Err("Cannot suspend terminated process");
        }
        ProcessState::Stopped => {
            return Err("Process already suspended");
        }
        _ => {}
    }

    let context = if current_pid() == Some(pid) {
        let mut saved: crate::sched::Context = unsafe { core::mem::zeroed() };
        unsafe { crate::sched::Context::save_to(&mut saved as *mut crate::sched::Context) };
        SuspendedContext {
            context: saved,
            suspended_at: crate::time::current_ticks(),
            previous_state: current_state,
        }
    } else {
        let saved_ctx = get_saved_interrupt_context(pid);
        SuspendedContext {
            context: saved_ctx,
            suspended_at: crate::time::current_ticks(),
            previous_state: current_state,
        }
    };

    SUSPENDED_CONTEXTS.write().insert(pid, context);
    *pcb.state.lock() = ProcessState::Stopped;

    if current_state == ProcessState::Running {
        if current_pid() == Some(pid) {
            let next = find_next_runnable_process();
            if let Some(next_pid) = next {
                let _ = context_switch(next_pid);
            }
        }
    }

    crate::sched::remove_from_run_queue(pid);

    crate::log_info!("Process {} suspended (was {:?})", pid, current_state);
    Ok(())
}

fn get_saved_interrupt_context(pid: Pid) -> crate::sched::Context {
    if let Some(ctx) = INTERRUPT_SAVED_CONTEXTS.read().get(&pid) {
        return ctx.clone();
    }

    crate::sched::Context::for_resume(
        get_process_stack_pointer(pid).unwrap_or(0x7FFF_FFFF_FFF8),
        get_process_instruction_pointer(pid).unwrap_or(0x0000_4000_0000),
    )
}

pub fn save_interrupt_context(pid: Pid, ctx: crate::sched::Context) {
    INTERRUPT_SAVED_CONTEXTS.write().insert(pid, ctx);
}

pub fn clear_interrupt_context(pid: Pid) {
    INTERRUPT_SAVED_CONTEXTS.write().remove(&pid);
}

pub fn save_fpu_state(pid: Pid) {
    let mut fpu = FpuState::default();
    fpu.save();
    INTERRUPT_SAVED_FPU_STATES.write().insert(pid, fpu);
}

pub fn restore_fpu_state(pid: Pid) {
    let fpu_copy = INTERRUPT_SAVED_FPU_STATES.read().get(&pid).cloned();
    if let Some(fpu) = fpu_copy {
        fpu.restore();
    }
}

pub fn clear_fpu_state(pid: Pid) {
    INTERRUPT_SAVED_FPU_STATES.write().remove(&pid);
}

pub fn has_saved_fpu_state(pid: Pid) -> bool {
    INTERRUPT_SAVED_FPU_STATES.read().contains_key(&pid)
}

pub fn init_fpu() {
    FpuState::init();
}

pub fn resume_process(pid: Pid) -> Result<(), &'static str> {
    let pcb = PROCESS_TABLE.find_by_pid(pid).ok_or("Process not found")?;

    let current_state = *pcb.state.lock();
    if current_state != ProcessState::Stopped {
        return Err("Process is not suspended");
    }

    let context = SUSPENDED_CONTEXTS.write().remove(&pid).ok_or("No saved context for process")?;

    // The registers are the saved context itself now, so resuming is handing it
    // back rather than copying it across.
    let restore_ctx = context.context;

    save_interrupt_context(pid, restore_ctx);
    *pcb.state.lock() = ProcessState::Ready;
    crate::sched::add_to_run_queue(pid);

    let suspend_duration = crate::time::current_ticks() - context.suspended_at;
    crate::log_info!("Process {} resumed after {} ticks", pid, suspend_duration);
    Ok(())
}

pub fn resume_and_switch(pid: Pid) -> Result<(), &'static str> {
    resume_process(pid)?;

    let ctx = INTERRUPT_SAVED_CONTEXTS.write().remove(&pid);

    if let Some(saved_ctx) = ctx {
        CURRENT_PID.store(pid, Ordering::SeqCst);
        saved_ctx.restore();
    } else {
        context_switch(pid)?;
    }

    Ok(())
}

fn get_process_stack_pointer(pid: Pid) -> Option<u64> {
    let pcb = PROCESS_TABLE.find_by_pid(pid)?;
    let mem = pcb.memory.lock();

    for vma in &mem.vmas {
        if vma.start.as_u64() >= 0x7000_0000_0000 {
            return Some(vma.end.as_u64() - 8);
        }
    }

    Some(0x7FFF_FFFF_FFF8)
}

fn get_process_instruction_pointer(pid: Pid) -> Option<u64> {
    let pcb = PROCESS_TABLE.find_by_pid(pid)?;
    let mem = pcb.memory.lock();

    if mem.code_start.as_u64() != 0 {
        Some(mem.code_start.as_u64())
    } else {
        Some(0x0000_4000_0000)
    }
}

fn find_next_runnable_process() -> Option<Pid> {
    let processes = PROCESS_TABLE.get_all_processes();
    for p in processes {
        let state = *p.state.lock();
        if state == ProcessState::Ready {
            return Some(p.pid);
        }
    }
    None
}
