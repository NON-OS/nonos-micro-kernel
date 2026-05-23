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
            rax: saved.rax,
            rbx: saved.rbx,
            rcx: saved.rcx,
            rdx: saved.rdx,
            rsi: saved.rsi,
            rdi: saved.rdi,
            rbp: saved.rbp,
            rsp: saved.rsp,
            r8: saved.r8,
            r9: saved.r9,
            r10: saved.r10,
            r11: saved.r11,
            r12: saved.r12,
            r13: saved.r13,
            r14: saved.r14,
            r15: saved.r15,
            rip: saved.rip,
            rflags: saved.rflags,
            suspended_at: crate::time::current_ticks(),
            previous_state: current_state,
        }
    } else {
        let saved_ctx = get_saved_interrupt_context(pid);
        SuspendedContext {
            rax: saved_ctx.rax,
            rbx: saved_ctx.rbx,
            rcx: saved_ctx.rcx,
            rdx: saved_ctx.rdx,
            rsi: saved_ctx.rsi,
            rdi: saved_ctx.rdi,
            rbp: saved_ctx.rbp,
            rsp: saved_ctx.rsp,
            r8: saved_ctx.r8,
            r9: saved_ctx.r9,
            r10: saved_ctx.r10,
            r11: saved_ctx.r11,
            r12: saved_ctx.r12,
            r13: saved_ctx.r13,
            r14: saved_ctx.r14,
            r15: saved_ctx.r15,
            rip: saved_ctx.rip,
            rflags: saved_ctx.rflags,
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

    crate::sched::Context {
        rax: 0,
        rbx: 0,
        rcx: 0,
        rdx: 0,
        rsi: 0,
        rdi: 0,
        rbp: 0,
        rsp: get_process_stack_pointer(pid).unwrap_or(0x7FFF_FFFF_FFF8),
        r8: 0,
        r9: 0,
        r10: 0,
        r11: 0,
        r12: 0,
        r13: 0,
        r14: 0,
        r15: 0,
        rip: get_process_instruction_pointer(pid).unwrap_or(0x0000_4000_0000),
        rflags: 0x202,
    }
}

pub fn save_interrupt_context(pid: Pid, ctx: crate::sched::Context) {
    INTERRUPT_SAVED_CONTEXTS.write().insert(pid, ctx);
}

pub fn clear_interrupt_context(pid: Pid) {
    INTERRUPT_SAVED_CONTEXTS.write().remove(&pid);
}

pub fn save_fpu_state(pid: Pid) {
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        pcb.fpu.lock().save();
        pcb.fpu_valid.store(true, Ordering::Release);
    }
}

pub fn restore_fpu_state(pid: Pid) {
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        if pcb.fpu_valid.load(Ordering::Acquire) {
            pcb.fpu.lock().restore();
        }
    }
}

pub fn clear_fpu_state(pid: Pid) {
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        pcb.fpu_valid.store(false, Ordering::Release);
    }
}

pub fn has_saved_fpu_state(pid: Pid) -> bool {
    PROCESS_TABLE
        .find_by_pid(pid)
        .map(|pcb| pcb.fpu_valid.load(Ordering::Acquire))
        .unwrap_or(false)
}

pub fn init_fpu() {
    let fpu = FpuState::default();
    fpu.restore();
}

pub fn resume_process(pid: Pid) -> Result<(), &'static str> {
    let pcb = PROCESS_TABLE.find_by_pid(pid).ok_or("Process not found")?;

    let current_state = *pcb.state.lock();
    if current_state != ProcessState::Stopped {
        return Err("Process is not suspended");
    }

    let context = SUSPENDED_CONTEXTS.write().remove(&pid).ok_or("No saved context for process")?;

    let restore_ctx = crate::sched::Context {
        rax: context.rax,
        rbx: context.rbx,
        rcx: context.rcx,
        rdx: context.rdx,
        rsi: context.rsi,
        rdi: context.rdi,
        rbp: context.rbp,
        rsp: context.rsp,
        r8: context.r8,
        r9: context.r9,
        r10: context.r10,
        r11: context.r11,
        r12: context.r12,
        r13: context.r13,
        r14: context.r14,
        r15: context.r15,
        rip: context.rip,
        rflags: context.rflags,
    };

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
