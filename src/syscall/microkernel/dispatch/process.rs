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
use crate::process::foreign::{
    sys_foreign_exec, sys_foreign_fork, sys_foreign_reply, sys_foreign_spawn, sys_foreign_start,
    sys_foreign_thread, sys_foreign_wait, sys_peer_copy, sys_peer_map, sys_peer_protect,
    sys_peer_tls, sys_peer_unmap,
};
use crate::syscall::microkernel::attest::sys_attest_status;
use crate::syscall::microkernel::attest_doc::sys_attest_doc;
use crate::syscall::microkernel::attest_entries::sys_attest_entries;
use crate::syscall::microkernel::battery::sys_battery_status;
use crate::syscall::microkernel::capsule_load::sys_capsule_load;
use crate::syscall::microkernel::capsule_verify::sys_capsule_verify;
use crate::syscall::microkernel::enrol_dev_root::{sys_dev_root_confirm, sys_dev_root_request};
use crate::syscall::microkernel::futex::{sys_futex_wait, sys_futex_wake};
use crate::syscall::microkernel::install_source::sys_install_source;
use crate::syscall::microkernel::local_sign::sys_local_sign;
use crate::syscall::microkernel::app_install::sys_app_install;
use crate::syscall::microkernel::enrol_local_root::sys_dev_root_local;
use crate::syscall::microkernel::local_verify::sys_local_verify;
use crate::syscall::microkernel::kill::sys_kill;
use crate::syscall::microkernel::memory::{sys_mmap, sys_munmap};
use crate::syscall::microkernel::numbers::*;
use crate::syscall::microkernel::proc_output::sys_proc_output;
use crate::syscall::microkernel::proc_stdin::{sys_proc_input, sys_stdin_read};
use crate::syscall::microkernel::process::{
    sys_args, sys_exit, sys_getpid, sys_pid_alive, sys_set_tls, sys_thread_spawn, sys_yield,
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
        SYS_ATTEST_ENTRIES => sys_attest_entries(a.a0, a.a1),
        SYS_INSTALL_SOURCE => sys_install_source(a.a0, a.a1, a.a2, a.a3),
        SYS_FOREIGN_SPAWN => sys_foreign_spawn(a.a0, a.a1),
        SYS_FOREIGN_START => sys_foreign_start(a.a0, a.a1, a.a2),
        SYS_FOREIGN_WAIT => sys_foreign_wait(a.a0, a.a1, a.a2),
        SYS_FOREIGN_REPLY => sys_foreign_reply(a.a0, a.a1),
        SYS_PEER_MAP => sys_peer_map(a.a0, a.a1, a.a2, a.a3),
        SYS_PEER_COPY => sys_peer_copy(a.a0, a.a1, a.a2, a.a3, a.a4),
        SYS_PEER_PROTECT => sys_peer_protect(a.a0, a.a1, a.a2, a.a3),
        SYS_FOREIGN_THREAD => sys_foreign_thread(a.a0, a.a1, a.a2, a.a3),
        SYS_PEER_TLS => sys_peer_tls(a.a0, a.a1),
        SYS_FOREIGN_FORK => sys_foreign_fork(a.a0),
        SYS_PEER_UNMAP => sys_peer_unmap(a.a0, a.a1, a.a2),
        SYS_FOREIGN_EXEC => sys_foreign_exec(a.a0, a.a1, a.a2),
        SYS_LOCAL_SIGN => sys_local_sign(a.a0, a.a1, a.a2, a.a3, a.a4),
        SYS_LOCAL_VERIFY => sys_local_verify(a.a0, a.a1, a.a2, a.a3, a.a4),
        SYS_APP_INSTALL => sys_app_install(a.a0, a.a1),
        SYS_DEV_ROOT_LOCAL => sys_dev_root_local(),
        SYS_DEV_ROOT_REQUEST => sys_dev_root_request(a.a0),
        SYS_DEV_ROOT_CONFIRM => sys_dev_root_confirm(a.a0),
        SYS_SPAWN_INSTANCE => sys_spawn_instance(a.a0, a.a1),
        SYS_TOOL_RUN => sys_tool_run(a.a0, a.a1, a.a2, a.a3),
        _ => return None,
    })
}
