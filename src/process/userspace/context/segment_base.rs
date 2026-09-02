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

//! FS and GS segment bases. Each is a 64-bit MSR written as two halves, low in
//! EAX and high in EDX. `KERNEL_GS_BASE` is the one SWAPGS exchanges in, so it
//! holds the per-CPU pointer the kernel finds itself with on syscall entry.

const MSR_FS_BASE: u32 = 0xC000_0100;
const MSR_GS_BASE: u32 = 0xC000_0101;
const MSR_KERNEL_GS_BASE: u32 = 0xC000_0102;

#[inline(always)]
pub fn write_fs_base(base: u64) {
    write(MSR_FS_BASE, base);
}

#[inline(always)]
pub fn write_gs_base(base: u64) {
    write(MSR_GS_BASE, base);
}

#[inline(always)]
pub fn write_kernel_gs_base(base: u64) {
    write(MSR_KERNEL_GS_BASE, base);
}

#[inline(always)]
pub fn read_fs_base() -> u64 {
    let (low, high): (u32, u32);
    // SAFETY: ek@nonos.systems - FS_BASE is architectural on every x86_64 part
    // and reading an MSR has no effect on machine state.
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") MSR_FS_BASE,
            out("eax") low,
            out("edx") high,
            options(nomem, nostack, preserves_flags),
        );
    }
    ((high as u64) << 32) | (low as u64)
}

#[inline(always)]
fn write(msr: u32, base: u64) {
    // SAFETY: ek@nonos.systems - all three are architectural segment-base MSRs
    // and accept any canonical value; the caller owns which base it is setting.
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") base as u32,
            in("edx") (base >> 32) as u32,
            options(nomem, nostack, preserves_flags),
        );
    }
}
