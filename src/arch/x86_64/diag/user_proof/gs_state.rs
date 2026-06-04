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

//! Snapshot of GS_BASE / KERNEL_GS_BASE and the per-CPU
//! `kernel_stack_top` mirror that the syscall fast path reads from
//! `gs:0x20`. The pre-iretq audit cross-checks this mirror against
//! `TSS.RSP0`; any mismatch means the scheduler updated only one
//! side and the next CPL=3 → CPL=0 trap (or `syscall`) lands on a
//! stale stack.

use crate::arch::x86_64::syscall::msr::{read_msr, IA32_GS_BASE, IA32_KERNEL_GS_BASE};

pub(super) struct GsState {
    pub base: u64,
    pub kernel_base: u64,
    pub rsp0: u64,
}

pub(super) fn read() -> GsState {
    GsState {
        base: read_msr(IA32_GS_BASE),
        kernel_base: read_msr(IA32_KERNEL_GS_BASE),
        rsp0: read_kernel_stack_top(),
    }
}

#[inline]
fn read_kernel_stack_top() -> u64 {
    let v: u64;
    // SAFETY: ek@nonos.systems — gs:0x20 is `kernel_stack_top` in
    // PerCpuData. The pre-iretq audit runs in kernel mode with GS
    // pointing at PerCpuData (no swapgs has happened on this exit
    // path yet).
    unsafe {
        core::arch::asm!(
            "mov {0}, gs:0x20",
            out(reg) v,
            options(nomem, nostack, preserves_flags),
        );
    }
    v
}
