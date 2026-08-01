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

use super::definition::Context;
use core::sync::atomic::{AtomicBool, Ordering};

const MAX_CPUS: usize = 256;
const INIT_FALSE: AtomicBool = AtomicBool::new(false);
static CONTEXT_JUST_RESTORED: [AtomicBool; MAX_CPUS] = [INIT_FALSE; MAX_CPUS];

#[inline]
fn current_cpu_index() -> usize {
    (crate::process::scheduler::api::current_cpu_id() as usize) % MAX_CPUS
}

pub(crate) fn set_restored_flag() {
    CONTEXT_JUST_RESTORED[current_cpu_index()].store(true, Ordering::SeqCst);
}

impl Context {
    #[unsafe(naked)]
    pub unsafe extern "C" fn save_to(ctx: *mut Context) {
        core::arch::naked_asm!(
            "mov [rdi], rax",
            "mov [rdi + 8], rbx",
            "mov [rdi + 16], rcx",
            "mov [rdi + 24], rdx",
            "mov [rdi + 32], rsi",
            "mov [rdi + 40], rdi",
            "mov [rdi + 48], rbp",
            "lea rax, [rsp + 8]",
            "mov [rdi + 56], rax",
            "mov [rdi + 64], r8",
            "mov [rdi + 72], r9",
            "mov [rdi + 80], r10",
            "mov [rdi + 88], r11",
            "mov [rdi + 96], r12",
            "mov [rdi + 104], r13",
            "mov [rdi + 112], r14",
            "mov [rdi + 120], r15",
            "mov rax, [rsp]",
            "mov [rdi + 128], rax",
            "pushfq",
            "pop rax",
            "mov [rdi + 136], rax",
            "ret",
        );
    }

    /// A context that resumes at `entry` on the stack `stack_top`, with every
    /// other register clear and interrupts enabled. Used where a thread has to be
    /// given a starting point rather than have one recovered from it, so shared
    /// code can ask for that without naming a register file.
    pub fn for_resume(stack_top: u64, entry: u64) -> Self {
        let mut ctx: Context = unsafe { core::mem::zeroed() };
        ctx.rsp = stack_top;
        ctx.rip = entry;
        // Bit 1 reads as one, bit 9 is IF: a resumed thread runs interruptible.
        ctx.rflags = 0x202;
        ctx
    }

    #[inline(never)]
    pub fn save() -> Self {
        let mut ctx: Context = unsafe { core::mem::zeroed() };
        unsafe { Self::save_to(&mut ctx as *mut Context) };
        ctx
    }

    pub fn was_just_restored() -> bool {
        CONTEXT_JUST_RESTORED[current_cpu_index()].swap(false, Ordering::SeqCst)
    }

    pub fn clear_restored_flag() {
        CONTEXT_JUST_RESTORED[current_cpu_index()].store(false, Ordering::SeqCst);
    }
}
