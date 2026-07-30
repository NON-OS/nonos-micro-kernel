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

//! The kernel-side context the scheduler saves before it switches away.
//!
//! This is the setjmp half of a switch, not the exception frame: a thread saves
//! where it stands, the scheduler runs something else, and a later restore lands
//! back on the instruction after the save with the stack it left. Only the
//! callee-saved half of the register file is here, because AAPCS64 already says
//! the caller does not expect x0 through x18 to survive a call, and `save_to` is
//! a call. Saving more would be dead stores the switch pays for every time.
//!
//! The x86_64 counterpart lives in `process::context::full` and saves the whole
//! register file, because its callers reach in for the interrupted frame as well.

use core::sync::atomic::Ordering;

use super::restored::{current_cpu_index, CONTEXT_JUST_RESTORED};

/// A saved kernel execution point: callee-saved registers, the frame and link
/// registers, and the stack pointer. Restoring it returns from `save_to` a second
/// time, which is how the scheduler tells a fresh save from a resume.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    /// Frame pointer.
    pub x29: u64,
    /// Link register, the address `restore_from` returns to.
    pub x30: u64,
    pub sp: u64,
}

impl Context {
    /// Save the calling frame into `ctx`.
    ///
    /// # Safety
    /// `ctx` must point to a writable, aligned `Context`.
    #[unsafe(naked)]
    pub unsafe extern "C" fn save_to(ctx: *mut Context) {
        // x0 holds `ctx`. Offsets follow the field order above, eight bytes each.
        core::arch::naked_asm!(
            "stp x19, x20, [x0, #0]",
            "stp x21, x22, [x0, #16]",
            "stp x23, x24, [x0, #32]",
            "stp x25, x26, [x0, #48]",
            "stp x27, x28, [x0, #64]",
            "stp x29, x30, [x0, #80]",
            // SP cannot be a source operand to `str`, so route it through a
            // caller-saved register, which by AAPCS64 is ours to clobber.
            "mov x1, sp",
            "str x1, [x0, #96]",
            "ret",
        );
    }

    /// Resume a saved point. Does not return to its own caller: it returns to
    /// whoever called `save_to`, which is the switch completing.
    ///
    /// # Safety
    /// `ctx` must be a context saved by `save_to` whose stack is still mapped and
    /// still owned by this thread.
    #[unsafe(naked)]
    pub unsafe extern "C" fn restore_from(ctx: *const Context) -> ! {
        // Put SP back before the final `ret`, so the frame the link register
        // returns into is the one that was saved with it.
        core::arch::naked_asm!(
            "ldp x19, x20, [x0, #0]",
            "ldp x21, x22, [x0, #16]",
            "ldp x23, x24, [x0, #32]",
            "ldp x25, x26, [x0, #48]",
            "ldp x27, x28, [x0, #64]",
            "ldp x29, x30, [x0, #80]",
            "ldr x1, [x0, #96]",
            "mov sp, x1",
            "ret",
        );
    }

    /// A context that resumes at `entry` on the stack `stack_top`, with every
    /// callee-saved register clear. Used where a thread has to be given a
    /// starting point rather than have one recovered from it, so shared code can
    /// ask for that without naming a register file.
    pub fn for_resume(stack_top: u64, entry: u64) -> Self {
        let mut ctx: Context = unsafe { core::mem::zeroed() };
        ctx.sp = stack_top;
        // The link register is where `restore_from` lands, which is this
        // context's resume point.
        ctx.x30 = entry;
        ctx
    }

    /// Save the calling frame and hand it back.
    #[inline(never)]
    pub fn save() -> Self {
        let mut ctx: Context = unsafe { core::mem::zeroed() };
        // SAFETY: `ctx` is a live, aligned local.
        unsafe { Self::save_to(&mut ctx as *mut Context) };
        ctx
    }

    /// Resume this context. Does not return.
    ///
    /// Checks the two fields that decide where control lands before handing the
    /// CPU over. A context with a null stack or entry is a corrupted save rather
    /// than a resumable thread, and jumping into it would land somewhere the
    /// caller did not choose, so the kernel stops instead.
    pub fn restore(&self) -> ! {
        if self.sp == 0 || self.x30 == 0 {
            crate::sys::serial::println(b"[FATAL] Context restore failed");
            crate::sys::serial::println(b"saved context has no stack or no entry");
            crate::arch::halt_loop()
        }
        // A kernel context resumes into EL1 on the stack it saved, so unlike the
        // user path there is no SPSR to sanitize here: the exception level and
        // mask bits are whatever the restoring code already holds.
        super::restored::set_restored_flag();
        // SAFETY: the fields that select the landing point were just checked, and
        // the stack belongs to the thread being resumed.
        unsafe { Self::restore_from(self as *const Context) }
    }

    /// True once per resume: it reads the flag and clears it, so the second
    /// return from `save_to` is distinguishable from the first exactly once.
    pub fn was_just_restored() -> bool {
        CONTEXT_JUST_RESTORED[current_cpu_index()].swap(false, Ordering::SeqCst)
    }

    pub fn clear_restored_flag() {
        CONTEXT_JUST_RESTORED[current_cpu_index()].store(false, Ordering::SeqCst);
    }
}
