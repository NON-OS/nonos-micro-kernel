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

use super::thread_group::ThreadGroup;
use super::types::{
    MemoryState, Pid, Priority, ProcessCredentials, ProcessIoStats, ProcessMemoryInfo,
    ProcessState, ProcessTimeInfo,
};
use crate::arch::context::{SavedUser, UserEntry};
use crate::capabilities::CapabilityToken;
use crate::process::mmap_va::MmapVa;
use crate::process::process_fd_table::ProcessFdTable;
use crate::process::signal::SignalState;
use alloc::{string::String, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicI32, AtomicU32, AtomicU64, Ordering};
use spin::{Mutex, RwLock};

pub struct ProcessControlBlock {
    pub pid: Pid,
    pub tgid: AtomicU32,
    pub ppid: AtomicU32,
    pub pgid: AtomicU32,
    pub sid: AtomicU32,
    pub name: Mutex<String>,
    pub state: Mutex<ProcessState>,
    pub priority: Mutex<Priority>,
    pub memory: Mutex<MemoryState>,
    pub thread_group: Option<Arc<ThreadGroup>>,
    pub argv: Mutex<Vec<String>>,
    pub envp: Mutex<Vec<String>>,
    // Truth for this process's authority. `caps_bits` is a derived
    // bitmap cache kept in sync by `process::caps` so the bitmap-only
    // readers (IPC routing, inheritance) stay an atomic load.
    pub capability_token: RwLock<Arc<CapabilityToken>>,
    pub caps_bits: AtomicU64,
    // One-shot gate for the verified-capsule manifest install. The
    // PCB is born holding the inheritance-derived token; `install_spawn`
    // flips this once to swap that token for the manifest-derived one.
    pub caps_manifest_installed: core::sync::atomic::AtomicBool,
    // Per-capsule revocation epoch. Bumped by `process::caps::revoke`
    // and minted into each token; the resolver compares the token's
    // epoch against this to invalidate authority that was minted
    // before the most recent revoke.
    pub revocation_epoch: AtomicU64,
    pub mmap_va: Mutex<MmapVa>,
    pub exit_code: AtomicI32,
    pub zk_proofs_generated: AtomicU64,
    pub zk_proving_time_ms: AtomicU64,
    pub zk_proofs_verified: AtomicU64,
    pub zk_verification_time_ms: AtomicU64,
    pub zk_circuits_compiled: AtomicU64,
    pub umask: Mutex<u32>,
    pub root_dir: Mutex<String>,
    pub cwd: Mutex<String>,
    pub clear_child_tid: AtomicU64,
    pub set_child_tid: AtomicU64,
    pub alarm_time_ms: AtomicU64,
    pub tls_base: AtomicU64,
    pub stack_base: AtomicU64,
    pub clone_flags: AtomicU64,
    pub start_time_ms: AtomicU64,
    pub fd_table: ProcessFdTable,
    pub signals: Mutex<SignalState>,
    pub time_info: Mutex<ProcessTimeInfo>,
    pub memory_info: Mutex<ProcessMemoryInfo>,
    pub creds: Mutex<ProcessCredentials>,
    pub io_stats: Mutex<ProcessIoStats>,
    pub tty_nr: AtomicU32,
    pub tty_pgrp: AtomicI32,
    pub flags: AtomicU64,
    pub nice: AtomicI32,
    pub thread_count: AtomicU32,
    pub pending_signals: AtomicU64,
    pub kstkesp: AtomicU64,
    pub kstkeip: AtomicU64,
    pub wchan: AtomicU64,
    pub exit_signal: AtomicI32,
    pub processor: AtomicU32,
    pub rt_priority: AtomicU32,
    pub policy: AtomicU32,
    pub no_new_privs: AtomicU32,
    pub seccomp: AtomicU32,
    pub cpus_allowed: AtomicU64,
    pub voluntary_switches: AtomicU64,
    pub involuntary_switches: AtomicU64,
    pub cr3: AtomicU64,
    pub io_bitmap: Mutex<[u8; 8192]>,
    // Kernel-only stack top installed in TSS RSP0 on context switch.
    // Allocated by `kernel_core::process_spawn::kernel_stack`. 0 means
    // unallocated; the scheduler hook treats this as "no user mode
    // expected" and refuses to dispatch a pending user entry.
    pub kernel_stack_top: AtomicU64,
    // Per-process snapshot of the per-cpu user rsp (gs:PCPU_USER_STACK).
    // Saved when parked, restored on resume so a preempted syscall's
    // SYSRET cannot pick up another process's user rsp.
    pub saved_user_stack: AtomicU64,
    // First-transition-to-user record consumed by the arch's enter-user
    // helper. On x86_64 this is the iretq 5-tuple; on aarch64/riscv64
    // it carries the per-arch entry shape (ELR/SP_EL0/SPSR + per-task
    // kernel sp on aarch64, sepc/sstatus/user_sp/kernel_sp on riscv64).
    // `None` for kernel threads.
    pub pending_user_entry: Mutex<Option<UserEntry>>,
    // Snapshot written by the per-arch trap-entry path when a user
    // task is preempted. On x86_64: 15 GPRs + iretq 5-tuple. On
    // aarch64: x0..x30 + SP_EL0/ELR_EL1/SPSR_EL1 + kernel sp top.
    // On riscv64: x1..x31 + sepc + sstatus + kernel sp top. Consumed
    // by the arch's resume-user helper.
    pub saved_user_context: Mutex<Option<SavedUser>>,
    // Per-PCB FP/SIMD slot for the non-x86 lazy-enable path. UnsafeCell
    // inside; accessed only on the CPU running this task. x86 keeps
    // its existing pid-keyed `FpuState` side-table, so no field there.
    #[cfg(target_arch = "aarch64")]
    pub arch_fpu: crate::arch::aarch64::fpu::PcbArchFpu,
    #[cfg(target_arch = "riscv64")]
    pub arch_fpu: crate::arch::riscv64::fpu::PcbArchFpu,
}

impl ProcessControlBlock {
    #[inline]
    pub fn pid(&self) -> Pid {
        self.pid
    }
    #[inline]
    pub fn parent_pid(&self) -> Pid {
        self.ppid.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn process_group(&self) -> Pid {
        self.pgid.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn session_id(&self) -> Pid {
        self.sid.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn thread_group_id(&self) -> Pid {
        self.tgid.load(Ordering::Acquire)
    }
    #[inline]
    pub fn exit_status(&self) -> i32 {
        self.exit_code.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn get_tls_base(&self) -> u64 {
        self.tls_base.load(Ordering::Acquire)
    }
    #[inline]
    pub fn set_tls_base(&self, base: u64) {
        self.tls_base.store(base, Ordering::Release);
    }
    #[inline]
    pub fn get_stack_base(&self) -> u64 {
        self.stack_base.load(Ordering::Acquire)
    }
    #[inline]
    pub fn set_stack_base(&self, base: u64) {
        self.stack_base.store(base, Ordering::Release);
    }
    #[inline]
    pub fn get_clear_child_tid(&self) -> u64 {
        self.clear_child_tid.load(Ordering::Acquire)
    }
    #[inline]
    pub fn set_clear_child_tid(&self, tidptr: u64) {
        self.clear_child_tid.store(tidptr, Ordering::Release);
    }
    #[inline]
    pub fn name(&self) -> String {
        self.name.lock().clone()
    }
    #[inline]
    pub fn get_name(&self) -> String {
        self.name.lock().clone()
    }

    #[inline]
    pub fn terminate(&self, code: i32) {
        self.exit_code.store(code, Ordering::Relaxed);
        *self.state.lock() = ProcessState::Terminated(code);
    }

    pub fn set_name(&self, new_name: &str) {
        let mut name = self.name.lock();
        name.clear();
        name.push_str(if new_name.len() > 256 { &new_name[..256] } else { new_name });
    }

    #[inline]
    pub fn is_thread(&self) -> bool {
        self.thread_group
            .as_ref()
            .map(|tg| tg.thread_count() > 1 || self.pid != tg.tgid)
            .unwrap_or(false)
    }

    #[inline]
    pub fn is_group_leader(&self) -> bool {
        self.thread_group.as_ref().map(|tg| tg.is_leader(self.pid)).unwrap_or(true)
    }

    const FLAG_CONTINUED: u64 = 1 << 63;

    #[inline]
    pub fn was_continued(&self) -> bool {
        (self.flags.load(Ordering::Acquire) & Self::FLAG_CONTINUED) != 0
    }

    #[inline]
    pub fn set_continued(&self) {
        self.flags.fetch_or(Self::FLAG_CONTINUED, Ordering::Release);
    }

    #[inline]
    pub fn clear_continued(&self) {
        self.flags.fetch_and(!Self::FLAG_CONTINUED, Ordering::Release);
    }
}
