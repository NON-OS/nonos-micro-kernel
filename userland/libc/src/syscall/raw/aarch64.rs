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

//! The aarch64 side of a system call.
//!
//! `svc #0` traps to EL1, where the kernel reads the call number from `x8` and
//! the arguments from `x0` through `x5`, and leaves the result in `x0`. That is
//! the same register contract `arch::aarch64::exceptions::handlers::svc` reads,
//! so the two have to be changed together.
//!
//! Only six arguments fit. The kernel's `SyscallArgs` carries six, so a seventh
//! would have nowhere to go on either side of the trap.

/// Trap into the kernel.
///
/// # Safety
/// The caller is invoking a kernel service: whether the arguments are sound is
/// the caller's business, and the kernel validates them again on its side.
#[inline]
pub(in crate::syscall) unsafe fn raw(
    num: i64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
) -> i64 {
    let ret: i64;
    // SAFETY: `svc` is the architected way to ask for a kernel service. The
    // operands name the registers the kernel reads, x0 is written back with the
    // result, and the call may touch memory the arguments point at, so it is not
    // marked `nomem`.
    unsafe {
        core::arch::asm!(
            "svc #0",
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            in("x5") a6,
            in("x8") num as u64,
            options(nostack)
        );
    }
    ret
}
