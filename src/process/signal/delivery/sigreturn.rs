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

use crate::process::signal::frame::parse_from_user_stack;
use crate::process::signal::SIGSEGV;
use crate::process::{current_pid, terminate_current_with_signal, with_process_mut};

pub fn sigreturn_current() -> ! {
    let pid = current_pid().unwrap_or(0);
    let user_rsp = read_user_rsp();
    let frame = match parse_from_user_stack(user_rsp) {
        Ok(f) => f,
        Err(_) => terminate_current_with_signal(SIGSEGV),
    };
    with_process_mut(pid, |pcb| {
        let mut sigs = pcb.signals.lock();
        // sigsuspend leaves the original mask in saved_mask so the
        // sigframe's suspended mask is overridden here.
        let restore_to = sigs.take_saved_mask().unwrap_or(frame.saved_blocked);
        sigs.set_blocked_mask(restore_to);
    });
    frame.saved_ctx.resume_user()
}

#[inline]
fn read_user_rsp() -> u64 {
    let rsp: u64;
    // SAFETY: ek@nonos.systems - reads `user_stack_saved` in PerCpuData, where
    // the syscall asm shim writes the user RSP on entry after `swapgs`. Kernel
    // GS is still active here because sysret/iretq has not happened yet.
    unsafe {
        core::arch::asm!(
            "mov {0}, gs:[{off}]",
            out(reg) rsp,
            off = const crate::smp::percpu::layout::USER_STACK_SAVED,
            options(nomem, nostack, preserves_flags),
        );
    }
    rsp
}
