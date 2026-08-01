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

// FROZEN: SHIM ONLY (Phase 1 kill list).
// Canonical scheduler authority lives under `src/process/scheduler` per
// CANONICAL_SUBSYSTEM_WINNER_MAP.md. No new code, no new exports, no new
// state may be added in this tree. Permitted work: migration extraction
// into the canonical winner, forwarding shims, deletion prep.
// See PHASE_1_KILL_LIST_AND_FREEZE_PLAN.md.

pub use crate::process::scheduler::api;
pub use crate::process::scheduler::cpu_stats;
pub use crate::process::scheduler::deadline;
pub use crate::process::scheduler::realtime;
pub use crate::process::scheduler::runqueue;
pub use crate::process::scheduler::task;
pub mod scheduler;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub use crate::process::context::Context;
pub use api::{current_cpu_id, current_scheduler, schedule, yield_cpu};
pub use cpu_stats::{get_cpu_stats, CpuStats};
pub use deadline::{
    bandwidth_utilization, get_stats as get_deadline_stats, has_runnable as has_deadline_tasks,
    init as deadline_init, run_deadline_tasks, spawn_deadline, task_count as deadline_task_count,
    AdmissionError, DeadlineStatsSnapshot,
};
pub use realtime::{
    has_realtime_tasks, init as realtime_init, pending_realtime_tasks, run_realtime_tasks,
    spawn_realtime,
};
pub use runqueue::RunQueue;
pub use scheduler::runnable_process_count as get_runnable_count;
pub use scheduler::{
    add_to_run_queue, clear_reschedule, enter, get, get_remaining_sleep, get_runnable_pids,
    get_scheduler_stats, init, is_in_run_queue, is_sleeping, need_reschedule,
    remove_from_run_queue, run, runnable_process_count, sleep_until, spawn, tick, wake_process,
    wakeup, yield_now, SchedulerStatsSnapshot,
};
pub use task::{CpuAffinity, Priority, Task};
