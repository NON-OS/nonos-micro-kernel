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

//! Hosting a guest: create it, build its address space, answer the calls
//! the kernel refuses on its behalf. The guest holds no capabilities, so
//! everything it can do passes through the supervisor that made it.

use crate::syscall::{
    call_raw, N_MK_FOREIGN_REPLY, N_MK_FOREIGN_SPAWN, N_MK_FOREIGN_START, N_MK_FOREIGN_THREAD,
    N_MK_FOREIGN_WAIT,
};

pub use crate::foreign_frame::ForeignFrame;

/// An empty, capability-free process supervised by this one. Returns its
/// pid, or a negative errno.
pub fn mk_foreign_spawn(name: &[u8]) -> i64 {
    call_raw(N_MK_FOREIGN_SPAWN, [name.as_ptr() as u64, name.len() as u64, 0, 0, 0, 0])
}

/// Give a built guest its entry point and make it runnable.
/// `rsp` of zero lets the kernel allocate an ordinary user stack; any
/// other value is the stack the supervisor built, which is what a program
/// expecting argv and an auxiliary vector needs.
pub fn mk_foreign_start(pid: u32, entry: u64, rsp: u64) -> i64 {
    call_raw(N_MK_FOREIGN_START, [pid as u64, entry, rsp, 0, 0, 0])
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
