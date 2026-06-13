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

use crate::syscall::{call_raw, N_MK_ARGS, N_MK_GETPID, N_MK_PID_ALIVE};

pub extern "C" fn mk_pid_alive(pid: u32) -> bool {
    call_raw(N_MK_PID_ALIVE, [pid as u64, 0, 0, 0, 0, 0]) == 1
}

pub extern "C" fn mk_getpid() -> u32 {
    let rc = call_raw(N_MK_GETPID, [0, 0, 0, 0, 0, 0]);
    if rc < 0 {
        0
    } else {
        rc as u32
    }
}

pub extern "C" fn mk_args(buf: *mut u8, len: usize) -> i64 {
    call_raw(N_MK_ARGS, [buf as u64, len as u64, 0, 0, 0, 0])
}
