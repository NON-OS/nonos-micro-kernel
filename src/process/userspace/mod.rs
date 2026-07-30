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

// Entering user mode is the one thing with no shared shape at all: iretq and
// sysret against a GDT selector here, `eret` to EL0 against SPSR there. The
// aarch64 counterpart lives in `arch::aarch64::context`. Only the layouts and
// the address-space constants below are common.
#[cfg(target_arch = "x86_64")]
pub mod api;
#[cfg(target_arch = "x86_64")]
pub mod asm;
pub mod constants;
#[cfg(target_arch = "x86_64")]
pub mod context;
#[cfg(target_arch = "x86_64")]
pub mod transitions;
pub mod types;

#[cfg(target_arch = "x86_64")]
pub use api::*;
pub use constants::{
    KERNEL_CS, KERNEL_DS, KERNEL_STACK_SIZE, USER_CODE_START, USER_CS, USER_DS, USER_HEAP_START,
    USER_RFLAGS, USER_STACK_BASE, USER_STACK_SIZE,
};
#[cfg(target_arch = "x86_64")]
pub use context::{
    clac, enable_smap, enable_smep, read_fs_base, stac, switch_context, switch_to_new_thread,
    write_fs_base, write_gs_base, write_kernel_gs_base,
};
#[cfg(target_arch = "x86_64")]
pub use transitions::{
    enable_smap as transitions_enable_smap, enable_smep as transitions_enable_smep, exec_process,
    jump_to_usermode, restore_user_context_iretq, return_to_usermode, sysret_to_usermode,
};

/// The window in which the kernel may touch user memory. Unlike everything
/// else here this is not x86 specific, because both architectures refuse by
/// default and both need the same narrow, explicit opening.
pub use crate::arch::user_access::with_user_access;
pub use types::{
    BlockReason, ExecContext, FpuState, InterruptFrame, KernelStack, ThreadControlBlock,
    ThreadState, UserContext,
};
