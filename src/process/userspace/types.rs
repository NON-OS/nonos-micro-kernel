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
use alloc::boxed::Box;

pub use super::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreadState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Sleeping = 3,
    Zombie = 4,
    Stopped = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockReason {
    Io,
    Lock,
    Futex(u64),
    Wait,
    Signal,
    Ipc,
}

#[repr(C, align(4096))]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: [0; KERNEL_STACK_SIZE] })
    }

    pub fn top(&self) -> u64 {
        let ptr = self.data.as_ptr() as u64;
        ptr + KERNEL_STACK_SIZE as u64
    }

    pub fn base(&self) -> u64 {
        self.data.as_ptr() as u64
    }
}

impl Default for KernelStack {
    fn default() -> Self {
        Self { data: [0; KERNEL_STACK_SIZE] }
    }
}

#[derive(Clone)]
#[repr(C, align(64))]
pub struct FpuState {
    pub data: [u8; 1024],
}

impl FpuState {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: [0; 1024] })
    }

    #[inline(always)]
    pub fn save(&mut self) {
        // SAFETY: FXSAVE saves the FPU/SSE state to a 512-byte memory region.
        // self.data is 1024 bytes and 64-byte aligned (repr(C, align(64))), which
        // exceeds the 16-byte alignment requirement for FXSAVE. The nostack option
        // is correct as FXSAVE only writes to the provided memory location.
        // FXSAVE writes the x86_64 legacy area. The aarch64 register file has a
        // different shape and is saved by `arch::aarch64::fpu`, so this body is
        // the one architecture that uses this layout.
        #[cfg(target_arch = "x86_64")]
        // SAFETY: the destination is this struct is own 512-byte align(64)
        // buffer, which meets FXSAVE is alignment and size requirement.
        unsafe {
            core::arch::asm!(
                "fxsave [{}]",
                in(reg) self.data.as_mut_ptr(),
                options(nostack, preserves_flags)
            );
        }
    }

    #[inline(always)]
    pub fn restore(&self) {
        // SAFETY: FXRSTOR restores FPU/SSE state from a 512-byte memory region.
        // self.data must have been previously populated by save() or be zeroed.
        // The alignment requirement (16 bytes) is satisfied by our align(64) repr.
        // The nostack option is correct as FXRSTOR only reads from memory.
        #[cfg(target_arch = "x86_64")]
        // SAFETY: reads back the same buffer `save` wrote, at the same alignment.
        unsafe {
            core::arch::asm!(
                "fxrstor [{}]",
                in(reg) self.data.as_ptr(),
                options(nostack, preserves_flags)
            );
        }
    }

    // Architectural default FPU/SSE state for a fresh thread. FNINIT sets
    // FCW=0x037F; MXCSR must be 0x1F80 (all SIMD exceptions masked, round to
    // nearest). Restoring a zeroed FXSAVE image instead leaves MXCSR=0, which
    // unmasks every SIMD exception and makes the first inexact result trap.
    #[inline(always)]
    pub fn init() {
        let mxcsr: u32 = 0x1F80;
        // x87 and SSE control words, so this is the x86_64 unit. The aarch64
        // FPCR is set where that FPU is brought up.
        #[cfg(target_arch = "x86_64")]
        // SAFETY: FNINIT resets the x87 unit; LDMXCSR loads the SSE control word
        // from the 4-byte `mxcsr` local. Neither touches the stack.
        unsafe {
            core::arch::asm!(
                "fninit",
                "ldmxcsr [{}]",
                in(reg) &mxcsr as *const u32,
                options(nostack),
            );
        }
    }
}

impl Default for FpuState {
    fn default() -> Self {
        Self { data: [0; 1024] }
    }
}

pub struct ThreadControlBlock {
    pub tid: u64,
    pub pid: u64,
    pub name: [u8; 32],
    pub state: ThreadState,
    pub context: CpuContext,
    pub kernel_stack: Box<KernelStack>,
    pub user_stack_top: u64,
    pub user_stack_base: u64,
    pub entry_point: u64,
    pub arg: u64,
    pub exit_code: i32,
    pub block_reason: Option<BlockReason>,
    pub wakeup_time: u64,
    pub fs_base: u64,
    pub gs_base: u64,
    pub kernel_gs_base: u64,
    pub fpu_state: Option<Box<FpuState>>,
    pub signal_mask: u64,
    pub pending_signals: u64,
}

impl ThreadControlBlock {
    pub fn new(tid: u64, pid: u64, entry: u64, user_stack: u64, arg: u64) -> Self {
        let mut tcb = Self {
            tid,
            pid,
            name: [0; 32],
            state: ThreadState::Ready,
            context: CpuContext::new(),
            kernel_stack: KernelStack::new(),
            user_stack_top: user_stack,
            user_stack_base: user_stack.saturating_sub(USER_STACK_SIZE as u64),
            entry_point: entry,
            arg,
            exit_code: 0,
            block_reason: None,
            wakeup_time: 0,
            fs_base: 0,
            gs_base: 0,
            kernel_gs_base: 0,
            fpu_state: None,
            signal_mask: 0,
            pending_signals: 0,
        };

        tcb.context.prepare_user_entry(
            entry,
            user_stack,
            USER_CS as u64,
            USER_DS as u64,
            USER_RFLAGS,
        );

        tcb.context.r15 = arg;

        tcb
    }

    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = core::cmp::min(bytes.len(), 31);
        self.name[..len].copy_from_slice(&bytes[..len]);
        self.name[len] = 0;
    }

    pub fn name(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(32);
        core::str::from_utf8(&self.name[..len]).unwrap_or("unknown")
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl InterruptFrame {
    pub fn for_user_entry(entry: u64, stack: u64) -> Self {
        Self { rip: entry, cs: USER_CS as u64, rflags: USER_RFLAGS, rsp: stack, ss: USER_DS as u64 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UserContext {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
    pub fs_base: u64,
    pub gs_base: u64,
}

#[repr(C)]
pub struct ExecContext {
    pub entry: u64,
    pub stack: u64,
    pub pid: u64,
    pub tid: u64,
    pub cr3: u64,
    pub argc: u64,
    pub argv: u64,
    pub envp: u64,
}
