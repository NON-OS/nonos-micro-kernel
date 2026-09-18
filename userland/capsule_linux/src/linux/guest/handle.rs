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

/// Where a guest's heap and its anonymous mappings start. Both are chosen
/// here rather than by the kernel, because a Linux program expects a Linux
/// address space and this is the only place that knows what that means.
pub const BRK_BASE: u64 = 0x0000_1000_0000;
pub const MMAP_BASE: u64 = 0x0000_2000_0000;

pub struct Guest {
    pub pid: u32,
    /// The program break, as `brk` moves it.
    pub brk: u64,
    /// The next address an anonymous mapping gets, growing upward.
    pub mmap_next: u64,
    pub fds: Vec<Fd>,
    /// Tids of this guest's threads, not counting itself.
    pub threads: Vec<u32>,
    /// Threads parked in a futex wait, with the word they wait on.
    pub waits: Vec<(u32, u64)>,
    /// What a relative path is relative to.
    pub cwd: Vec<u8>,
    /// Where the guest last asked its thread pointer to be set.
    pub fs_base: u64,
    /// Set once the guest asks to end, so the loop can drop it.
    pub exited: Option<i32>,
}

impl Guest {
    pub fn new(pid: u32) -> Self {
        Guest {
            pid,
            brk: BRK_BASE,
            mmap_next: MMAP_BASE,
            fds: Fd::standard(),
            threads: Vec::new(),
            waits: Vec::new(),
            cwd: alloc::vec![b'/'],
            fs_base: 0,
            exited: None,
        }
    }
}
