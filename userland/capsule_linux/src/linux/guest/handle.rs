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

//! One hosted process, and everything this personality remembers about it.

use alloc::vec::Vec;

use super::fd::Fd;

pub struct Guest {
    pub pid: u32,
    /// The program break, as `brk` moves it.
    pub brk: u64,
    /// The next address an anonymous mapping gets, growing upward.
    pub mmap_next: u64,
    pub fds: Vec<Fd>,
    /// Every span this capsule has backed for the guest, in the order
    /// it did so. Fork copies exactly this list.
    pub regions: Vec<crate::linux::guest::Region>,
    /// Pipe buffers, named by index from the descriptors at each end.
    pub pipes: Vec<Vec<u8>>,
    /// Children this guest has forked, for wait to report on.
    pub children: Vec<u32>,
    /// Tids of this guest's threads, not counting itself.
    pub threads: Vec<u32>,
    /// Threads parked in a futex wait, with the word they wait on.
    pub waits: Vec<(u32, u64)>,
    /// The display connection, when the guest has opened one.
    pub display: crate::linux::unix::Conn,
    /// The Wayland objects that connection has created.
    pub objects: crate::linux::wayland::Objects,
    /// What those objects describe, and the surface it reaches.
    pub scene: crate::linux::wayland::Scene,
    /// Which signals the guest installed a handler for. Nothing is ever
    /// raised against them; see `call::signal`.
    pub handlers: [bool; 64],
    /// What a relative path is relative to.
    pub cwd: Vec<u8>,
    /// Names this guest has resolved, each with the address it was given.
    pub automap: Vec<(Vec<u8>, [u8; 4])>,
    /// Where the guest last asked its thread pointer to be set.
    pub fs_base: u64,
    /// Set once the guest asks to end, so the loop can drop it.
    pub exited: Option<i32>,
    /// The personality, which hosts every guest it spawns.
    pub parent: u32,
    /// Process group and session.
    pub pgid: u32,
    pub sid: u32,
    /// Remembered, not enforced: the store does not apply it when it creates a
    /// file.
    pub umask: u16,
}
