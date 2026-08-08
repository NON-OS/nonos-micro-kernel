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

use super::constants::{USER_CS, USER_DS, USER_RFLAGS};
use super::types::{InterruptFrame, UserContext};

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn jump_to_usermode(entry: u64, stack: u64, arg: u64) -> ! {
    // Iretq frame order (top of stack last, popped by CPU first):
    //   ss      = USER_DS
    //   rsp     = caller-supplied user stack (rsi)
    //   rflags  = USER_RFLAGS
    //   cs      = USER_CS
    //   rip     = caller-supplied entry (rdi)
    core::arch::naked_asm!(
        "mov ax, {ds}",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "push {ds}",
        "push rsi",
        "push {flags}",
        "push {cs}",
        "push rdi",
        "mov rdi, rdx",
        "xor rax, rax",
        "xor rbx, rbx",
        "xor rcx, rcx",
        "xor rdx, rdx",
        "xor rsi, rsi",
        "xor rbp, rbp",
        "xor r8, r8",
        "xor r9, r9",
        "xor r10, r10",
        "xor r11, r11",
        "xor r12, r12",
        "xor r13, r13",
        "xor r14, r14",
        "xor r15, r15",
        "swapgs",
        "iretq",
        cs = const USER_CS as u64,
        ds = const USER_DS as u64,
        flags = const USER_RFLAGS,
    );
}

pub unsafe fn return_to_usermode(frame: *const InterruptFrame) -> ! {
    let f = unsafe { &*frame };
    const USER_SPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;
    if f.rip > USER_SPACE_MAX || f.rsp > USER_SPACE_MAX || f.rsp == 0 {
        crate::sys::serial::println(b"[FATAL] Invalid user frame");
        crate::arch::halt_loop()
    }

    {
        use core::sync::atomic::{AtomicU32, Ordering};
        static FIRST_ENTRY_SHOWN: AtomicU32 = AtomicU32::new(0);
        if FIRST_ENTRY_SHOWN.fetch_add(1, Ordering::Relaxed) < 64 {
            crate::sys::serial::print(b"[USER-FIRST] pid=");
            crate::sys::serial::print_hex(crate::process::current_pid().unwrap_or(0) as u64);
            crate::sys::serial::print(b" rip=");
            crate::sys::serial::print_hex(f.rip);
            crate::sys::serial::println(b"");
        }
    }

    #[cfg(feature = "nonos-user-entry-proof")]
    {
        let cr3 = crate::arch::paging::read_root();
        let cpu_id = crate::smp::percpu::current().cpu_id;
        if !crate::arch::x86_64::diag::assert_user_entry(cr3, f.rip, f.rsp, cpu_id) {
            crate::sys::serial::println(b"[USER-PROOF] halting before iretq");
            crate::arch::halt_loop();
        }
    }

    unsafe { return_to_usermode_asm(frame) }
}

#[unsafe(naked)]
unsafe extern "C" fn return_to_usermode_asm(frame: *const InterruptFrame) -> ! {
    core::arch::naked_asm!(
        "mov rsp, rdi",
        "mov ax, {ds}",
        "mov ds, ax",
        "mov es, ax",
        "swapgs",
        "iretq",
        ds = const USER_DS as u64,
    );
}

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn sysret_to_usermode(rip: u64, rsp: u64, rflags: u64, retval: u64) -> ! {
    core::arch::naked_asm!(
        "mov rax, rdi",
        "shr rax, 47",
        "cmp rax, 0",
        "je 2f",
        "cmp rax, 0x1FFFF",
        "jne 1f",
        "2:",
        "mov rax, rcx",
        "mov rcx, rdi",
        "mov r11, rdx",
        "mov rsp, rsi",
        "xor rbx, rbx",
        "xor rdx, rdx",
        "xor rsi, rsi",
        "xor rdi, rdi",
        "xor rbp, rbp",
        "xor r8, r8",
        "xor r9, r9",
        "xor r10, r10",
        "xor r12, r12",
        "xor r13, r13",
        "xor r14, r14",
        "xor r15, r15",
        "swapgs",
        "sysretq",
        "1:",
        "ud2",
    );
}

// Restore a captured CPL=3 user context and iretq into user mode.
//
// `ctx` points at a `UserContext` whose first 160 bytes are laid out
// to match the trap-entry trampoline's pushed frame:
//
//     offset    field
//     ------    ---------
//        0      r15
//        8      r14
//       16      r13
//       24      r12
//       32      r11
//       40      r10
//       48      r9
//       56      r8
//       64      rdi
//       72      rsi
//       80      rbp
//       88      rbx
//       96      rdx
//      104      rcx
//      112      rax
//      120      rip
//      128      cs
//      136      rflags
//      144      rsp
//      152      ss
//
// fs_base / gs_base at offsets 160 / 168 are present in `UserContext`
// but not used by this restore path.
//
// The function builds the CPU's iretq 5-tuple on the current kernel
// stack from the saved frame, restores the GPRs, performs `swapgs`
// when returning to CPL=3, and `iretq`s. Caller invariants:
//   - TSS.RSP0 must already be the next user-mode trap stack
//     (`gdt::set_kernel_stack(cpu, kernel_stack_top)` is the producer
//     and is called by the scheduler before this function).
//   - CR3 must already be the target capsule's address space.
//   - Caller is responsible for FPU restore.
#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn restore_user_context_iretq(ctx: *const UserContext) -> ! {
    core::arch::naked_asm!(
        // Build iretq 5-tuple on the current kernel stack: ss, rsp, rflags, cs, rip
        // (pushed in reverse so the last push is rip, which the CPU pops first).
        "push qword ptr [rdi + 152]", // ss
        "push qword ptr [rdi + 144]", // user rsp
        "push qword ptr [rdi + 136]", // rflags
        "push qword ptr [rdi + 128]", // cs
        "push qword ptr [rdi + 120]", // rip
        // Restore GPRs. rdi must be restored last because we are still
        // using it as the context pointer.
        "mov r15, [rdi + 0]",
        "mov r14, [rdi + 8]",
        "mov r13, [rdi + 16]",
        "mov r12, [rdi + 24]",
        "mov r11, [rdi + 32]",
        "mov r10, [rdi + 40]",
        "mov r9,  [rdi + 48]",
        "mov r8,  [rdi + 56]",
        "mov rsi, [rdi + 72]",
        "mov rbp, [rdi + 80]",
        "mov rbx, [rdi + 88]",
        "mov rdx, [rdi + 96]",
        "mov rcx, [rdi + 104]",
        "mov rax, [rdi + 112]",
        "mov rdi, [rdi + 64]", // restore rdi LAST
        // Returning to CPL=3 means the CS we just pushed has RPL bits
        // set. Inspect the saved CS at [rsp+8] (rip is at [rsp+0]).
        "test byte ptr [rsp + 8], 3",
        "jz 1f",
        "swapgs",
        "1:",
        "iretq",
    );
}
