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

use super::errnos::{ERRNO_INVAL, ERRNO_PERM};
use crate::process::signal::{SIGINT, SIGKILL, SIGTERM};
use crate::process::{
    current_pid, get_parent_pid, get_process, terminate_current_with_signal, ProcessState,
};
use crate::syscall::caps::current_caps_or_default;

pub fn sys_kill(pid: u64, sig: u64) -> i64 {
    if sig != SIGINT as u64 && sig != SIGTERM as u64 && sig != SIGKILL as u64 {
        return ERRNO_INVAL;
    }
    let target = pid as u32;
    let caller = current_pid().unwrap_or(0);
    let is_parent = caller != 0 && get_parent_pid(target) == Some(caller);
    if !is_parent && !current_caps_or_default().can_signal() {
        return ERRNO_PERM;
    }
    if !pid_alive(target) {
        return 0;
    }
    if current_pid() == Some(target) {
        terminate_current_with_signal(sig as u8);
    }
    crate::process::exit::teardown(target, 128 + sig as i32, true);
    0
}

fn pid_alive(pid: u32) -> bool {
    match get_process(pid) {
        Some(pcb) => {
            !matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
        }
        None => false,
    }
}
