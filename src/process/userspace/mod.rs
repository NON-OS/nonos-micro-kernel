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

pub mod api;
pub mod asm;
pub mod constants;
pub mod context;
pub mod transitions;
pub mod types;

pub use api::*;
pub use constants::{
    KERNEL_CS, KERNEL_DS, KERNEL_STACK_SIZE, USER_CODE_START, USER_CS, USER_DS, USER_HEAP_START,
    USER_RFLAGS, USER_STACK_BASE, USER_STACK_SIZE,
};
pub use context::{
    clac, enable_smap, enable_smep, read_fs_base, stac, switch_context, switch_to_new_thread,
    with_user_access, write_fs_base, write_gs_base, write_kernel_gs_base,
};
pub use transitions::{
    enable_smap as transitions_enable_smap, enable_smep as transitions_enable_smep, exec_process,
    jump_to_usermode, restore_user_context_iretq, return_to_usermode, sysret_to_usermode,
};
pub use types::{
    BlockReason, ExecContext, FpuState, InterruptFrame, KernelStack, ThreadControlBlock,
    ThreadState, UserContext,
};
