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

//! The one word of a guest's state that is not in the saved frame.

/// The user stack pointer.
#[inline]
pub fn user_rsp() -> u64 {
    let rsp: u64;
    /*
     * SAFETY: eK@nonos.systems - reads `user_stack_saved` in PerCpuData
     * at a compile-time offset. Kernel GS is still active on this path:
     * neither sysret nor iretq has run, which is the same condition the
     * sigreturn path relies on for the same read.
     */
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
