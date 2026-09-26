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

//! Hosting a guest: create it, build its address space, answer the calls the
//! kernel refuses on its behalf.

use crate::syscall::{
    call_raw, N_MK_FOREIGN_EXEC, N_MK_FOREIGN_FORK, N_MK_FOREIGN_REPLY, N_MK_FOREIGN_SPAWN,
    N_MK_FOREIGN_START,
    N_MK_FOREIGN_THREAD, N_MK_FOREIGN_WAIT,
};

pub use crate::foreign_frame::ForeignFrame;

/// An empty, capability-free process supervised by this one. Returns its
/// pid, or a negative errno.
pub fn mk_foreign_spawn(name: &[u8]) -> i64 {
    call_raw(N_MK_FOREIGN_SPAWN, [name.as_ptr() as u64, name.len() as u64, 0, 0, 0, 0])
}

/// Give a built guest its entry point and make it runnable.
pub fn mk_foreign_start(pid: u32, entry: u64, rsp: u64) -> i64 {
    call_raw(N_MK_FOREIGN_START, [pid as u64, entry, rsp, 0, 0, 0])
}

/// Make a guest runnable on the state it already carries, which is
/// what a forked child was born holding.
pub fn mk_foreign_resume(pid: u32) -> i64 {
    call_raw(N_MK_FOREIGN_START, [pid as u64, 0, 0, 0, 0, 0])
}

/// A second process holding a guest's register state, with zero in its return
/// register.
pub fn mk_foreign_fork(pid: u32) -> i64 {
    call_raw(N_MK_FOREIGN_FORK, [pid as u64, 0, 0, 0, 0, 0])
}

/// Replace the program a parked guest is running.
pub fn mk_foreign_exec(pid: u32, entry: u64, rsp: u64) -> i64 {
    call_raw(N_MK_FOREIGN_EXEC, [pid as u64, entry, rsp, 0, 0, 0])
}

/// A thread in a guest, sharing its address space. `tls` is the FS base
/// it wakes with, which a C runtime reads before anything else.
pub fn mk_foreign_thread(pid: u32, entry: u64, rsp: u64, tls: u64) -> i64 {
    call_raw(N_MK_FOREIGN_THREAD, [pid as u64, entry, rsp, tls, 0, 0])
}

/// Block until a guest of this process makes a call the kernel refuses,
/// then take its register frame. `timeout_ms` of zero waits forever.
pub fn mk_foreign_wait(out: &mut ForeignFrame, timeout_ms: u64) -> i64 {
    let ptr = out as *mut ForeignFrame as u64;
    let len = core::mem::size_of::<ForeignFrame>() as u64;
    call_raw(N_MK_FOREIGN_WAIT, [ptr, len, timeout_ms, 0, 0, 0])
}

/// Answer one parked guest with the value its `rax` receives.
pub fn mk_foreign_reply(pid: u32, value: u64) -> i64 {
    call_raw(N_MK_FOREIGN_REPLY, [pid as u64, value, 0, 0, 0, 0])
}
