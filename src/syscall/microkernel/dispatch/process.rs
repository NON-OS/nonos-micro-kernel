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

use super::args::Args;
use crate::syscall::microkernel::attest::sys_attest_status;
use crate::syscall::microkernel::attest_doc::sys_attest_doc;
use crate::syscall::microkernel::enrol_dev_root::{sys_dev_root_confirm, sys_dev_root_request};
use crate::syscall::microkernel::battery::sys_battery_status;
use crate::syscall::microkernel::capsule_load::sys_capsule_load;
use crate::syscall::microkernel::capsule_verify::sys_capsule_verify;
use crate::syscall::microkernel::futex::{sys_futex_wait, sys_futex_wake};
use crate::syscall::microkernel::kill::sys_kill;
use crate::syscall::microkernel::memory::{sys_mmap, sys_munmap};
use crate::syscall::microkernel::numbers::*;
use crate::syscall::microkernel::proc_output::sys_proc_output;
use crate::syscall::microkernel::proc_stdin::{sys_proc_input, sys_stdin_read};
use crate::syscall::microkernel::process::{
    sys_args, sys_exit, sys_getpid, sys_pid_alive, sys_set_tls, sys_sleep_ms, sys_spawn,
    sys_thread_spawn,
    sys_yield,
};
use crate::syscall::microkernel::procstat::sys_proc_stat;
use crate::syscall::microkernel::spawn_instance::sys_spawn_instance;
use crate::syscall::microkernel::stdout_write::sys_stdout_write;
use crate::syscall::microkernel::store_write::sys_store_write;
use crate::syscall::microkernel::time::{
    sys_time_adjust, sys_time_millis, sys_time_monotonic, sys_time_rtc,
};
use crate::syscall::microkernel::tool_run::sys_tool_run;
use crate::syscall::microkernel::wait::sys_wait;

pub(super) fn handle(nr: u64, a: Args) -> Option<i64> {
    Some(match nr {
        SYS_MMAP => sys_mmap(a.a0, a.a1 as usize, a.a2 as u32, a.a3 as u32),
        SYS_MUNMAP => sys_munmap(a.a0, a.a1 as usize),
        SYS_SPAWN => sys_spawn(a.a0, a.a1 as usize),
        SYS_CAPSULE_LOAD => sys_capsule_load(a.a0),
        SYS_CAPSULE_VERIFY => sys_capsule_verify(a.a0, a.a1),
        SYS_EXIT => sys_exit(a.a0 as i32),
        SYS_PID_ALIVE => sys_pid_alive(a.a0 as u32),
        SYS_WAIT => sys_wait(a.a0, a.a1),
        SYS_KILL => sys_kill(a.a0, a.a1),
        SYS_GETPID => sys_getpid(),
        SYS_ARGS => sys_args(a.a0, a.a1 as usize),
        SYS_THREAD_SPAWN => sys_thread_spawn(a.a0, a.a1),
        SYS_SET_TLS => sys_set_tls(a.a0),
        SYS_YIELD => sys_yield(),
        SYS_FUTEX_WAIT => sys_futex_wait(a.a0, a.a1 as u32, a.a2),
        SYS_FUTEX_WAKE => sys_futex_wake(a.a0, a.a1),
        SYS_SLEEP_MS => sys_sleep_ms(a.a0),
        SYS_TIME_MILLIS => sys_time_millis(),
        SYS_TIME_MONOTONIC => sys_time_monotonic(),
        SYS_TIME_RTC => sys_time_rtc(a.a0),
        SYS_TIME_ADJUST => sys_time_adjust(a.a0),
        SYS_BATTERY_STATUS => sys_battery_status(),
        SYS_PROC_STAT => sys_proc_stat(a.a0, a.a1),
        SYS_PROC_OUTPUT => sys_proc_output(a.a0, a.a1, a.a2 as usize),
        SYS_PROC_INPUT => sys_proc_input(a.a0, a.a1, a.a2 as usize),
        SYS_STDIN_READ => sys_stdin_read(a.a0, a.a1 as usize),
        SYS_STDOUT_WRITE => sys_stdout_write(a.a0, a.a1),
        SYS_STORE_WRITE => sys_store_write(a.a0, a.a1, a.a2),
        SYS_ATTEST_STATUS => sys_attest_status(a.a0),
        SYS_ATTEST_DOC => sys_attest_doc(a.a0, a.a1, a.a2),
        SYS_DEV_ROOT_REQUEST => sys_dev_root_request(a.a0),
        SYS_DEV_ROOT_CONFIRM => sys_dev_root_confirm(a.a0),
        SYS_SPAWN_INSTANCE => sys_spawn_instance(a.a0, a.a1),
        SYS_TOOL_RUN => sys_tool_run(a.a0, a.a1, a.a2, a.a3),
        _ => return None,
    })
}
