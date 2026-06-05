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
use crate::syscall::microkernel::memory::{sys_mmap, sys_munmap};
use crate::syscall::microkernel::numbers::*;
use crate::syscall::microkernel::battery::sys_battery_status;
use crate::syscall::microkernel::process::{sys_exit, sys_pid_alive, sys_spawn, sys_yield};
use crate::syscall::microkernel::time::{sys_time_millis, sys_time_rtc};

pub(super) fn handle(nr: u64, a: Args) -> Option<i64> {
    Some(match nr {
        SYS_MMAP => sys_mmap(a.a0, a.a1 as usize, a.a2 as u32, a.a3 as u32),
        SYS_MUNMAP => sys_munmap(a.a0, a.a1 as usize),
        SYS_SPAWN => sys_spawn(a.a0, a.a1 as usize),
        SYS_EXIT => sys_exit(a.a0 as i32),
        SYS_PID_ALIVE => sys_pid_alive(a.a0 as u32),
        SYS_YIELD => sys_yield(),
        SYS_TIME_MILLIS => sys_time_millis(),
        SYS_TIME_RTC => sys_time_rtc(a.a0),
        SYS_BATTERY_STATUS => sys_battery_status(),
        _ => return None,
    })
}
