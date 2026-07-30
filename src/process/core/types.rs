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

use crate::memory::addr::VirtAddr;
use crate::sched::Context;
use alloc::vec::Vec;
use core::sync::atomic::AtomicU64;

pub type Pid = u32;
pub type Tid = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    New,
    Ready,
    Running,
    Sleeping,
    Stopped,
    Zombie(i32),
    Terminated(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Idle,
    Low,
    Normal,
    High,
    RealTime,
}

#[derive(Debug, Clone)]
pub struct Vma {
    pub start: VirtAddr,
    pub end: VirtAddr,
    pub flags: u64,
}

#[derive(Debug)]
pub struct MemoryState {
    pub code_start: VirtAddr,
    pub code_end: VirtAddr,
    pub vmas: Vec<Vma>,
    pub resident_pages: AtomicU64,
    pub(crate) next_va: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct IsolationFlags {
    pub no_network: bool,
    pub no_filesystem: bool,
    pub no_ipc: bool,
    pub no_devices: bool,
    pub no_signals: bool,
    pub memory_isolated: bool,
}

impl Default for IsolationFlags {
    fn default() -> Self {
        Self {
            no_network: true,
            no_filesystem: true,
            no_ipc: true,
            no_devices: true,
            no_signals: true,
            memory_isolated: true,
        }
    }
}

/// A suspended thread: the kernel execution point it stopped at, plus when and
/// what it was doing. The registers live in `Context` rather than being copied
/// out field by field, so there is one definition of a saved thread per arch and
/// this struct does not have to know which arch it is on.
#[derive(Debug, Clone)]
pub struct SuspendedContext {
    pub context: Context,
    pub suspended_at: u64,
    pub previous_state: ProcessState,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessTimeInfo {
    pub utime: u64,
    pub stime: u64,
    pub cutime: u64,
    pub cstime: u64,
    pub start_time: u64,
    pub guest_time: u64,
    pub cguest_time: u64,
    pub delayacct_blkio_ticks: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessMemoryInfo {
    pub vm_peak: u64,
    pub vm_size: u64,
    pub vm_hwm: u64,
    pub vm_rss: u64,
    pub vm_data: u64,
    pub vm_stack: u64,
    pub vm_exe: u64,
    pub vm_lib: u64,
    pub vm_pte: u64,
    pub rss_anon: u64,
    pub rss_file: u64,
    pub rss_shmem: u64,
    pub vsize: u64,
    pub rsslim: u64,
    pub startcode: u64,
    pub endcode: u64,
    pub startstack: u64,
    pub start_data: u64,
    pub end_data: u64,
    pub start_brk: u64,
    pub arg_start: u64,
    pub arg_end: u64,
    pub env_start: u64,
    pub env_end: u64,
    pub minflt: u64,
    pub cminflt: u64,
    pub majflt: u64,
    pub cmajflt: u64,
}

pub const NGROUPS_MAX: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct ProcessCredentials {
    pub uid: u32,
    pub euid: u32,
    pub suid: u32,
    pub fsuid: u32,
    pub gid: u32,
    pub egid: u32,
    pub sgid: u32,
    pub fsgid: u32,
    pub groups: [u32; NGROUPS_MAX],
    pub ngroups: usize,
}

impl Default for ProcessCredentials {
    fn default() -> Self {
        Self {
            uid: 0,
            euid: 0,
            suid: 0,
            fsuid: 0,
            gid: 0,
            egid: 0,
            sgid: 0,
            fsgid: 0,
            groups: [0; NGROUPS_MAX],
            ngroups: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessIoStats {
    pub rchar: u64,
    pub wchar: u64,
    pub syscr: u64,
    pub syscw: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub cancelled_write_bytes: u64,
}

#[inline]
pub fn align_up(v: u64, a: u64) -> u64 {
    (v + (a - 1)) & !(a - 1)
}

#[inline]
pub fn overlaps(vmas: &[Vma], start: VirtAddr, len: usize) -> bool {
    let s = start.as_u64();
    let e = s + len as u64;
    for v in vmas {
        let vs = v.start.as_u64();
        let ve = v.end.as_u64();
        if !(e <= vs || s >= ve) {
            return true;
        }
    }
    false
}
