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

// Forwarding shim. Scheduler authority lives in `src/process/scheduler`.
// Old `crate::sched::scheduler::*` paths still resolve through these aliases
// so out-of-tree callers do not have to be touched in lockstep with each cut.

pub use crate::process::scheduler::core;
pub use crate::process::scheduler::dispatch as process;
pub use crate::process::scheduler::module_tasks;
pub use crate::process::scheduler::preemption;
pub use crate::process::scheduler::selection;
pub use crate::process::scheduler::stats;
pub use crate::process::scheduler::types;

pub use core::{enter, get, init, run, spawn};
pub use module_tasks::{
    force_kill_module_tasks, has_running_tasks, spawn_module_task, terminate_module_tasks,
};
pub use preemption::{clear_reschedule, need_reschedule, tick, yield_now};
pub use process::{
    add_to_run_queue, get_remaining_sleep, get_runnable_pids, is_in_run_queue, is_sleeping,
    remove_from_run_queue, runnable_process_count, sleep_until, sleep_until_unless_woken,
    wake_process, wake_token, wakeup,
};
pub use stats::get_scheduler_stats;
pub use types::{ModuleTaskError, ModuleTaskResult, Scheduler, SchedulerStatsSnapshot};
