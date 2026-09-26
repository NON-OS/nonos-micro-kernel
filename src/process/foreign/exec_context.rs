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

//! The registers a program starts on.

use crate::arch::context::SavedUser;
use crate::process::userspace::{USER_CS, USER_DS, USER_RFLAGS};

/// Everything zero but the entry point, the stack and the constants the ABI
/// fixes.
pub(super) fn fresh(entry: u64, rsp: u64) -> SavedUser {
    SavedUser {
        rax: 0,
        rbx: 0,
        rcx: 0,
        rdx: 0,
        rsi: 0,
        rdi: 0,
        rbp: 0,
        r8: 0,
        r9: 0,
        r10: 0,
        r11: 0,
        r12: 0,
        r13: 0,
        r14: 0,
        r15: 0,
        rip: entry,
        rsp,
        rflags: USER_RFLAGS,
        cs: USER_CS as u64,
        ss: USER_DS as u64,
        // The thread pointer belongs to the runtime that is being replaced.
        fs_base: 0,
        gs_base: 0,
    }
}
