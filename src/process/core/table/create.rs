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

use super::super::pcb::ProcessControlBlock;
use super::super::types::{MemoryState, Pid, Priority, ProcessIoStats, ProcessState};
use super::inherit::compute_inherited_caps;
use super::types::{CURRENT_PID, NEXT_PID, PROCESS_TABLE};
use crate::memory::addr::VirtAddr;
use crate::process::process_fd_table::ProcessFdTable;
use alloc::{string::String, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub fn create_process(
    name: &str,
    state: ProcessState,
    prio: Priority,
) -> Result<Pid, &'static str> {
    create_process_with_mem(name, state, prio, 0)
}

pub fn create_process_with_mem(
    name: &str,
    state: ProcessState,
    prio: Priority,
    mem_kb: u64,
) -> Result<Pid, &'static str> {
    if name.is_empty() {
        return Err("empty name");
    }
    let pid = NEXT_PID.fetch_add(1, Ordering::Relaxed);
    let parent_pid = CURRENT_PID.load(Ordering::Relaxed);
    let caps = compute_inherited_caps(pid, parent_pid);
    let pcb = build_pcb(pid, parent_pid, name, state, prio, mem_kb / 4, caps)?;
    crate::process::address_space::lifecycle::allocate(&pcb)?;
    crate::process::caps::rebind_address_space(&pcb).ok_or("boot session nonce missing")?;
    PROCESS_TABLE.add(pcb);
    Ok(pid)
}

fn build_pcb(
    pid: Pid,
    ppid: Pid,
    name: &str,
    st: ProcessState,
    pr: Priority,
    pg: u64,
    caps: u64,
) -> Result<Arc<ProcessControlBlock>, &'static str> {
    use super::super::types::{ProcessCredentials, ProcessMemoryInfo, ProcessTimeInfo};
    use crate::process::signal::SignalState;
    use core::sync::atomic::{AtomicI32, AtomicU32 as AU32};
    let token = crate::process::caps::new_token(pid, caps).ok_or("boot session nonce missing")?;
    Ok(Arc::new(ProcessControlBlock {
        pid,
        tgid: AtomicU32::new(pid),
        ppid: AtomicU32::new(ppid),
        pgid: AtomicU32::new(pid),
        sid: AtomicU32::new(pid),
        name: spin::Mutex::new(String::from(name)),
        state: spin::Mutex::new(st),
        priority: spin::Mutex::new(pr),
        memory: spin::Mutex::new(MemoryState {
            code_start: VirtAddr::new(0),
            code_end: VirtAddr::new(0),
            vmas: Vec::new(),
            resident_pages: AtomicU64::new(pg),
            next_va: 0x0000_4000_0000,
        }),
        thread_group: None,
        argv: spin::Mutex::new(Vec::new()),
        envp: spin::Mutex::new(Vec::new()),
        capability_token: spin::RwLock::new(token),
        caps_bits: AtomicU64::new(caps),
        caps_manifest_installed: core::sync::atomic::AtomicBool::new(false),
        revocation_epoch: AtomicU64::new(0),
        mmap_va: spin::Mutex::new(crate::process::mmap_va::MmapVa::new()),
        exit_code: AtomicI32::new(0),
        zk_proofs_generated: AtomicU64::new(0),
        zk_proving_time_ms: AtomicU64::new(0),
        zk_proofs_verified: AtomicU64::new(0),
        zk_verification_time_ms: AtomicU64::new(0),
        zk_circuits_compiled: AtomicU64::new(0),
        umask: spin::Mutex::new(0o022),
        root_dir: spin::Mutex::new(String::from("/")),
        cwd: spin::Mutex::new(String::from("/")),
        clear_child_tid: AtomicU64::new(0),
        set_child_tid: AtomicU64::new(0),
        alarm_time_ms: AtomicU64::new(0),
        tls_base: AtomicU64::new(0),
        stack_base: AtomicU64::new(0),
        clone_flags: AtomicU64::new(0),
        start_time_ms: AtomicU64::new(crate::time::timestamp_millis()),
        fd_table: ProcessFdTable::new(),
        signals: spin::Mutex::new(SignalState::default()),
        time_info: spin::Mutex::new(ProcessTimeInfo::default()),
        memory_info: spin::Mutex::new(ProcessMemoryInfo::default()),
        creds: spin::Mutex::new(ProcessCredentials::default()),
        io_stats: spin::Mutex::new(ProcessIoStats::default()),
        tty_nr: AU32::new(0),
        tty_pgrp: AtomicI32::new(-1),
        flags: AtomicU64::new(0),
        nice: AtomicI32::new(0),
        thread_count: AU32::new(1),
        pending_signals: AtomicU64::new(0),
        kstkesp: AtomicU64::new(0),
        kstkeip: AtomicU64::new(0),
        wchan: AtomicU64::new(0),
        exit_signal: AtomicI32::new(17),
        processor: AU32::new(0),
        rt_priority: AU32::new(0),
        policy: AU32::new(0),
        no_new_privs: AU32::new(0),
        seccomp: AU32::new(0),
        cpus_allowed: AtomicU64::new(!0),
        voluntary_switches: AtomicU64::new(0),
        involuntary_switches: AtomicU64::new(0),
        cr3: AtomicU64::new(0),
        io_bitmap: spin::Mutex::new([0xFF; 8192]),
        kernel_stack_top: AtomicU64::new(0),
        syscall_user_rsp: AtomicU64::new(0),
        pending_user_entry: spin::Mutex::new(None),
        saved_user_context: spin::Mutex::new(None),
        #[cfg(target_arch = "aarch64")]
        arch_fpu: crate::arch::aarch64::fpu::PcbArchFpu::zeroed(),
        #[cfg(target_arch = "riscv64")]
        arch_fpu: crate::arch::riscv64::fpu::PcbArchFpu::zeroed(),
    }))
}
