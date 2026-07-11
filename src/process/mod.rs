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

pub mod address_space;
pub mod alarm;
pub mod api;
pub mod caps;
pub mod context;
pub mod core;
pub mod exit;
pub mod fd_table;
pub mod fd_types;
pub mod manager;
pub mod mmap_va;
pub mod operations_exec;
pub mod process_fd_table;
pub mod scheduler;
pub mod signal;
pub mod types;
pub mod userspace;

pub use context as nonos_context;
pub use context::CpuContext;
pub use core as nonos_core;
pub use core::{
    allocate_tid, clear_fpu_state, clear_interrupt_context, context_switch, create_process,
    current_pid, current_process, get_process_stats, get_process_table, has_saved_fpu_state,
    init_fpu, init_process_management, is_process_active, is_process_active_by_id, isolate_process,
    restore_fpu_state, save_fpu_state, save_interrupt_context, suspend_process, Pid, Priority,
    ProcessControlBlock, ProcessCredentials, ProcessManagementStats, ProcessMemoryInfo,
    ProcessState, ProcessTable, ProcessTimeInfo, ThreadGroup, CURRENT_PID,
    INTERRUPT_SAVED_CONTEXTS, INTERRUPT_SAVED_FPU_STATES, PROCESS_TABLE,
};
pub use manager::{
    get_process_manager, init_process_manager, is_manager_initialized, ProcessManager,
};
pub use operations_exec::{
    enumerate_all_processes, exec_fn, exec_process, exit_current_process, exit_thread,
    get_all_processes, get_current_process, get_thread_count, get_thread_ids, set_root, set_umask,
    update_memory_usage,
};
pub use signal::SignalState;
pub use types::Process;

pub use api::{
    current_tid, current_uid, exit_status, get_current_pty, get_parent_pid, get_process,
    get_tty_pgrp, get_uid, last_pid, list_all_pids, release_controlling_tty, resume_process,
    resume_process_by_pid, set_comm, set_controlling_tty, set_cwd, set_tty_pgrp, stop_process,
    terminate_current_with_signal, with_process, with_process_mut,
};
