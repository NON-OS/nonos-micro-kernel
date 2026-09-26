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

//! The register state a parked guest is holding, kept where a fork can reach
//! it.

use crate::arch::context::SavedUser;
use crate::process::userspace::{USER_CS, USER_DS};

pub use crate::arch::x86_64::asm::SYSCALL_FRAME_WORDS as FRAME_WORDS;

/*
 * rax is pushed last and sits at zero; the callee-saved five go first, so they
 * count down from the top and move with the constant.
 */
const RAX: usize = 0;
const R8: usize = 1;
const R9: usize = 2;
const R10: usize = 3;
const RCX: usize = 4;
const R11: usize = 5;
const RBP: usize = 6;
const RDI: usize = 7;
const RSI: usize = 8;
const RDX: usize = 9;
const RBX: usize = FRAME_WORDS - 5;
const R12: usize = FRAME_WORDS - 4;
const R13: usize = FRAME_WORDS - 3;
const R14: usize = FRAME_WORDS - 2;
const R15: usize = FRAME_WORDS - 1;
const _: () = assert!(RDX < RBX);

pub fn capture(frame: &[u64; FRAME_WORDS], user_rsp: u64) -> SavedUser {
    SavedUser {
        rax: frame[RAX],
        rbx: frame[RBX],
        rcx: frame[RCX],
        rdx: frame[RDX],
        rsi: frame[RSI],
        rdi: frame[RDI],
        rbp: frame[RBP],
        r8: frame[R8],
        r9: frame[R9],
        r10: frame[R10],
        r11: frame[R11],
        r12: frame[R12],
        r13: frame[R13],
        r14: frame[R14],
        r15: frame[R15],
        rsp: user_rsp,
        /*
         * SYSCALL leaves the resume point in rcx and the flags in r11, so the
         * frame carries both under those names.
         */
        rip: frame[RCX],
        rflags: frame[R11],
        cs: USER_CS as u64,
        ss: USER_DS as u64,
        fs_base: 0,
        gs_base: 0,
    }
}
