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

// Canonical scheduler authority. Dispatcher loop (core), realtime and
// deadline tier schedulers, preemption path, PID run queue (dispatch)
// and the sched_setattr / setpolicy / setaffinity / setnice policy
// registry the syscall layer talks to. All lifted out of src/sched
// during Phase 1.

extern crate alloc;

pub mod api;
pub mod contract;
pub mod core;
pub mod cpu_stats;
pub mod deadline;
pub mod dispatch;
pub mod module_tasks;
pub mod policy;
pub mod policy_types;
pub mod preemption;
pub mod realtime;
pub mod runqueue;
pub mod selection;
pub mod stats;
pub mod task;
pub mod types;

pub use policy::{
    get_affinity, get_ioprio, get_nice, get_online_cpu_count, get_online_cpu_mask, get_policy,
    get_priority, get_sched_attr, get_stats, init_process, remove_process, set_affinity,
    set_ioprio, set_nice, set_policy, set_priority, set_sched_attr,
};
pub use policy_types::{
    decode_ioprio_class, decode_ioprio_level, encode_ioprio, LinuxSchedAttr, SchedAttr, SchedParam,
    SchedPolicyStats, IOPRIO_CLASS_BE, IOPRIO_CLASS_IDLE, IOPRIO_CLASS_NONE, IOPRIO_CLASS_RT,
    IOPRIO_WHO_PGRP, IOPRIO_WHO_PROCESS, IOPRIO_WHO_USER, NICE_DEFAULT, NICE_MAX, NICE_MIN,
    SCHED_BATCH, SCHED_DEADLINE, SCHED_FIFO, SCHED_IDLE, SCHED_NORMAL, SCHED_PRIORITY_MAX,
    SCHED_PRIORITY_MIN, SCHED_RR,
};
pub use runqueue::RunQueue;
