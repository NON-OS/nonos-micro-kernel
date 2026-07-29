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

use crate::process::nonos_context::CpuContext;

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn switch_context(current_ctx: *mut CpuContext, next_ctx: *const CpuContext) {
    core::arch::naked_asm!(
        // Save current context
        // RDI = current_ctx, RSI = next_ctx

        // Save callee-saved registers
        "mov [rdi + 0], r15",
        "mov [rdi + 8], r14",
        "mov [rdi + 16], r13",
        "mov [rdi + 24], r12",
        "mov [rdi + 32], rbx",
        "mov [rdi + 40], rbp",
        // Save instruction pointer (return address on stack)
        "mov rax, [rsp]",
        "mov [rdi + 48], rax",
        // Save stack pointer (after return address)
        "lea rax, [rsp + 8]",
        "mov [rdi + 56], rax",
        // Save RFLAGS
        "pushfq",
        "pop rax",
        "mov [rdi + 64], rax",
        // Restore next context
        // Load callee-saved registers
        "mov r15, [rsi + 0]",
        "mov r14, [rsi + 8]",
        "mov r13, [rsi + 16]",
        "mov r12, [rsi + 24]",
        "mov rbx, [rsi + 32]",
        "mov rbp, [rsi + 40]",
        // Load RFLAGS
        "mov rax, [rsi + 64]",
        "push rax",
        "popfq",
        // Load stack pointer
        "mov rsp, [rsi + 56]",
        // Jump to saved instruction pointer
        "jmp [rsi + 48]",
    );
}

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn switch_to_new_thread(ctx: *const CpuContext, kernel_stack_top: u64) {
    core::arch::naked_asm!(
        // RDI = ctx, RSI = kernel_stack_top

        // Set up kernel stack
        "mov rsp, rsi",
        // Load context
        "mov r15, [rdi + 0]",
        "mov r14, [rdi + 8]",
        "mov r13, [rdi + 16]",
        "mov r12, [rdi + 24]",
        "mov rbx, [rdi + 32]",
        "mov rbp, [rdi + 40]",
        // Load RFLAGS
        "mov rax, [rdi + 64]",
        "push rax",
        "popfq",
        // Jump to entry point
        "jmp [rdi + 48]",
    );
}

const MSR_FS_BASE: u32 = 0xC0000100;
const MSR_GS_BASE: u32 = 0xC0000101;
const MSR_KERNEL_GS_BASE: u32 = 0xC0000102;

#[inline(always)]
pub fn write_fs_base(base: u64) {
    // SAFETY: WRMSR is safe to call in kernel mode. MSR_FS_BASE (0xC0000100) is
    // the standard x86_64 MSR for the FS segment base. The value written is
    // split into EAX (low 32 bits) and EDX (high 32 bits) per the x86_64 ABI.
    // nomem/nostack options are correct as this only accesses MSRs.
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") MSR_FS_BASE,
            in("eax") base as u32,
            in("edx") (base >> 32) as u32,
            options(nomem, nostack)
        );
    }
}

#[inline(always)]
pub fn read_fs_base() -> u64 {
    let low: u32;
    let high: u32;
    // SAFETY: RDMSR is safe to call in kernel mode. MSR_FS_BASE (0xC0000100) is
    // the standard x86_64 MSR for the FS segment base. The value is returned in
    // EAX (low 32 bits) and EDX (high 32 bits) per the x86_64 ABI.
    // nomem/nostack options are correct as this only reads MSRs.
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") MSR_FS_BASE,
            out("eax") low,
            out("edx") high,
            options(nomem, nostack)
        );
    }
    ((high as u64) << 32) | (low as u64)
}

#[inline(always)]
pub fn write_gs_base(base: u64) {
    // SAFETY: WRMSR is safe to call in kernel mode. See write_fs_base for details.
    // MSR_GS_BASE (0xC0000101) is the standard x86_64 MSR for the GS segment base.
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") MSR_GS_BASE,
            in("eax") base as u32,
            in("edx") (base >> 32) as u32,
            options(nomem, nostack)
        );
    }
}

#[inline(always)]
pub fn write_kernel_gs_base(base: u64) {
    // SAFETY: WRMSR is safe to call in kernel mode. MSR_KERNEL_GS_BASE (0xC0000102)
    // holds the value that will be swapped into GS base on SWAPGS instruction.
    // This is used to establish the kernel's per-CPU data pointer on syscall entry.
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") MSR_KERNEL_GS_BASE,
            in("eax") base as u32,
            in("edx") (base >> 32) as u32,
            options(nomem, nostack)
        );
    }
}

const CR4_SMEP: u64 = 1 << 20;
const CR4_SMAP: u64 = 1 << 21;

pub fn enable_smep() {
    // SAFETY: CR4 manipulation is safe in kernel mode. SMEP (bit 20) prevents
    // the kernel from executing code in user-space pages, which is a security
    // feature. The caller has verified SMEP support via CPUID.
    unsafe {
        let cr4: u64;
        core::arch::asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack));
        core::arch::asm!("mov cr4, {}", in(reg) cr4 | CR4_SMEP, options(nomem, nostack));
    }
    crate::log::info!("[USERSPACE] SMEP enabled");
}

pub fn enable_smap() {
    // SAFETY: CR4 manipulation is safe in kernel mode. SMAP (bit 21) prevents
    // the kernel from reading/writing user-space pages unless explicitly allowed
    // via STAC/CLAC. The caller has verified SMAP support via CPUID.
    unsafe {
        let cr4: u64;
        core::arch::asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack));
        core::arch::asm!("mov cr4, {}", in(reg) cr4 | CR4_SMAP, options(nomem, nostack));
    }
    crate::log::info!("[USERSPACE] SMAP enabled");
}

#[inline(always)]
pub fn stac() {
    // SAFETY: STAC (Set AC flag) is safe to call. It temporarily allows kernel
    // access to user pages when SMAP is enabled. Must be paired with CLAC to
    // re-enable SMAP protection. preserves_flags is correct as STAC only sets AC.
    unsafe {
        core::arch::asm!("stac", options(nomem, nostack, preserves_flags));
    }
}

#[inline(always)]
pub fn clac() {
    // SAFETY: CLAC (Clear AC flag) is safe to call. It re-enables SMAP protection
    // after user memory access. Should be called after any STAC to restore security.
    // preserves_flags is correct as CLAC only clears AC.
    unsafe {
        core::arch::asm!("clac", options(nomem, nostack, preserves_flags));
    }
}

// Kept as the name the usercopy paths already call. The window itself, and
// the guard that closes it, belong to the arch boundary.
#[inline(always)]
pub fn with_user_access<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    crate::arch::user_access::with_user_access(f)
}
