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
use super::super::types::Pid;
use alloc::{sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicU32, Ordering};
use spin::RwLock;

const INIT_PID: AtomicU32 = AtomicU32::new(0);

pub struct CurrentPid {
    slots: [AtomicU32; crate::smp::MAX_CPUS],
}

impl CurrentPid {
    pub const fn new() -> Self {
        Self { slots: [INIT_PID; crate::smp::MAX_CPUS] }
    }

    #[inline]
    fn slot(&self) -> &AtomicU32 {
        &self.slots[crate::smp::cpu_id()]
    }

    #[inline]
    pub fn load(&self, order: Ordering) -> u32 {
        self.slot().load(order)
    }

    #[inline]
    pub fn store(&self, value: u32, order: Ordering) {
        self.slot().store(value, order);
    }

    #[inline]
    pub fn swap(&self, value: u32, order: Ordering) -> u32 {
        self.slot().swap(value, order)
    }
}

#[derive(Default)]
pub struct ProcessTable {
    pub(super) inner: RwLock<Vec<Arc<ProcessControlBlock>>>,
}

impl ProcessTable {
    pub fn add(&self, pcb: Arc<ProcessControlBlock>) {
        self.inner.write().push(pcb);
    }
    pub fn get_all_processes(&self) -> Vec<Arc<ProcessControlBlock>> {
        self.inner.read().clone()
    }
    pub fn find_by_pid(&self, pid: Pid) -> Option<Arc<ProcessControlBlock>> {
        self.inner.read().iter().find(|p| p.pid == pid).cloned()
    }
    pub fn is_active_name(&self, name: &str) -> bool {
        self.inner.read().iter().any(|p| p.name.lock().as_str() == name)
    }
    pub fn is_active_pid(&self, pid: u64) -> bool {
        self.inner.read().iter().any(|p| p.pid as u64 == pid)
    }
    pub fn get_children_of(&self, parent_pid: Pid) -> Vec<Arc<ProcessControlBlock>> {
        self.inner.read().iter().filter(|p| p.parent_pid() == parent_pid).cloned().collect()
    }
    pub fn has_children(&self, pid: Pid) -> bool {
        self.inner.read().iter().any(|p| p.parent_pid() == pid)
    }
    pub fn get_process(&self, pid: Pid) -> Option<Arc<ProcessControlBlock>> {
        self.find_by_pid(pid)
    }
}

pub static PROCESS_TABLE: ProcessTable = ProcessTable { inner: RwLock::new(Vec::new()) };
pub static CURRENT_PID: CurrentPid = CurrentPid::new();
pub(super) static NEXT_PID: AtomicU32 = AtomicU32::new(1);

static PID_ALLOC_LOCK: spin::Mutex<()> = spin::Mutex::new(());

pub fn allocate_tid() -> Option<Pid> {
    const MAX_ATTEMPTS: u32 = 65536;
    let _guard = PID_ALLOC_LOCK.lock();
    let mut attempts = 0;
    loop {
        let current = NEXT_PID.load(Ordering::SeqCst);
        let next = if current >= u32::MAX - 1 { 1 } else { current + 1 };
        NEXT_PID.store(next, Ordering::SeqCst);
        let pid = if current == 0 { 1 } else { current };
        if !PROCESS_TABLE.is_active_pid(pid as u64) {
            return Some(pid);
        }
        attempts += 1;
        if attempts >= MAX_ATTEMPTS {
            crate::log::error!("[PROCESS] PID space exhausted after {} attempts", MAX_ATTEMPTS);
            return None;
        }
    }
}
