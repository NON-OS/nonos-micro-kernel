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

/// x86_64 SYSCALL trampoline. This lives in a naked assembly shim
/// instead of inline asm so Rust treats it like a normal C call and
/// preserves the full caller-clobbered ABI around the syscall leaf.
///
/// Syscall register layout expected by the NONOS kernel entry shim:
///   rax = number, rdi = a1, rsi = a2, rdx = a3,
///   r10 = a4    (rcx is clobbered by SYSCALL itself),
///   r8  = a5,   r9  = a6
///
/// The Rust caller invokes this helper with SysV arguments:
///   rdi = number, rsi = a1, rdx = a2, rcx = a3, r8 = a4, r9 = a5,
///   [rsp + 8] = a6
///
/// # Safety
/// This is the leaf at which user-mode hands control to the kernel.
/// The caller is responsible for argument meaning. Bad pointers do not
/// produce undefined behavior here; the kernel returns `-EFAULT`.
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "C" fn raw_syscall_asm(
    _num: i64,
    _a1: u64,
    _a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
    _a6: u64,
) -> i64 {
    core::arch::naked_asm!(
        "mov rax, rdi",
        "mov rdi, rsi",
        "mov rsi, rdx",
        "mov rdx, rcx",
        "mov r10, r8",
        "mov r8, r9",
        "mov r9, [rsp + 8]",
        "syscall",
        "ret",
    );
}

#[inline]
#[cfg(target_arch = "x86_64")]
pub(super) unsafe fn raw(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> i64 {
    raw_syscall_asm(num, a1, a2, a3, a4, a5, a6)
}

#[inline]
#[cfg(not(target_arch = "x86_64"))]
pub(super) unsafe fn raw(
    _num: i64,
    _a1: u64,
    _a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
    _a6: u64,
) -> i64 {
    -38
}
