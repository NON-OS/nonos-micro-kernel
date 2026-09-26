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

//! Leaving the kernel on a program the thread was not running when it entered.

use crate::arch::context::SavedUser;
use crate::process::signal::SIGSEGV;
use crate::process::userspace::{restore_user_context_iretq, USER_CS, USER_DS};

const USER_VA_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

pub(super) fn enter(pid: u32) -> ! {
    let ctx = crate::process::with_process(pid, |pcb| pcb.saved_user_context.lock().take());
    match ctx.flatten() {
        Some(c) if sane(&c) => resume(c),
        // Nothing to run.
        _ => crate::process::terminate_current_with_signal(SIGSEGV),
    }
}

fn resume(ctx: SavedUser) -> ! {
    /*
     * The scheduler installs the control block's base on every switch, and
     * this path deliberately does not go through the scheduler, so the
     * register is written here as well.
     */
    crate::arch::context::set_user_tls(ctx.fs_base);
    /*
     * SAFETY: eK@nonos.systems - `ctx` is a local, so it outlives the
     * five pushes the restore makes below rsp. Its selectors are the
     * user pair and rip/rsp are in the low half, checked above; the
     * address space is this pid's own, which is already on cr3 because
     * this thread is the one running.
     */
    unsafe { restore_user_context_iretq(&ctx) }
}

fn sane(c: &SavedUser) -> bool {
    c.cs == USER_CS as u64
        && c.ss == USER_DS as u64
        && c.rip <= USER_VA_MAX
        && c.rsp <= USER_VA_MAX
        && c.rsp != 0
}
