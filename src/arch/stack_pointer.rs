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

//! The current stack pointer.
//!
//! Read for entropy and for stack-depth checks, never to build a pointer that
//! is then dereferenced: the value is only meaningful inside the frame that
//! asked for it.

/// The calling frame's stack pointer.
#[inline]
pub(crate) fn stack_pointer() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let sp: u64;
        // SAFETY: reads RSP into a general register. No memory is touched and
        // no flag is written.
        unsafe {
            core::arch::asm!("mov {}, rsp", out(reg) sp, options(nomem, nostack, preserves_flags));
        }
        sp
    }
    #[cfg(target_arch = "aarch64")]
    {
        let sp: u64;
        // SAFETY: `mov Xd, SP` names the stack pointer in the instruction
        // rather than as an operand, which is the only way inline asm can read
        // it. No memory is touched and no flag is written.
        unsafe {
            core::arch::asm!("mov {}, sp", out(reg) sp, options(nomem, nostack, preserves_flags));
        }
        sp
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    0
}
