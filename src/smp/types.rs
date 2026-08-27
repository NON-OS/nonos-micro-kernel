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

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuState {
    Offline = 0,
    Starting = 1,
    Online = 2,
    GoingOffline = 3,
    Halted = 4,
}

impl From<u8> for CpuState {
    fn from(v: u8) -> Self {
        match v {
            0 => CpuState::Offline,
            1 => CpuState::Starting,
            2 => CpuState::Online,
            3 => CpuState::GoingOffline,
            4 => CpuState::Halted,
            _ => CpuState::Offline,
        }
    }
}

#[repr(C, align(64))]
pub struct CpuDescriptor {
    /// Written once by the BSP before the AP is started, read afterwards by
    /// that AP and by any CPU sending it an IPI. Atomic because the
    /// descriptors live in a shared static: a plain field written through a
    /// cast from `&self` is a data race the compiler may reorder or drop.
    pub cpu_id: AtomicU32,
    pub apic_id: AtomicU32,
    state: AtomicU32,
    pub numa_node: u32,
    pub stack_base: AtomicU64,
    pub stack_size: AtomicUsize,
    pub idle_cycles: AtomicU64,
    pub total_cycles: AtomicU64,
    pub current_pid: AtomicU32,
    /// Set by this CPU's idle loop while it is halted with nothing to run.
    /// Read by `wake_idle_cpu` to pick a CPU worth interrupting. Only ever
    /// written by its owner, so a stale read costs a redundant IPI or one
    /// timer tick of latency, never a lost task: the idle loop rechecks the
    /// run queue after every wake.
    pub idle: AtomicBool,
    pub preempt_disable_count: AtomicU32,
    pub in_interrupt: AtomicBool,
    pub last_error: AtomicU32,
    _pad: [u8; 4],
}

impl CpuDescriptor {
    pub const fn new() -> Self {
        Self {
            cpu_id: AtomicU32::new(0),
            apic_id: AtomicU32::new(0),
            state: AtomicU32::new(CpuState::Offline as u32),
            numa_node: 0,
            stack_base: AtomicU64::new(0),
            stack_size: AtomicUsize::new(0),
            idle_cycles: AtomicU64::new(0),
            total_cycles: AtomicU64::new(0),
            current_pid: AtomicU32::new(0),
            idle: AtomicBool::new(false),
            preempt_disable_count: AtomicU32::new(0),
            in_interrupt: AtomicBool::new(false),
            last_error: AtomicU32::new(0),
            _pad: [0; 4],
        }
    }

    pub fn state(&self) -> CpuState {
        CpuState::from(self.state.load(Ordering::Acquire) as u8)
    }

    pub fn set_state(&self, state: CpuState) {
        self.state.store(state as u32, Ordering::Release);
    }

    pub fn is_online(&self) -> bool {
        self.state() == CpuState::Online
    }

    /// Get CPU ID
    pub fn get_cpu_id(&self) -> u32 {
        self.cpu_id.load(Ordering::Acquire)
    }

    /// Get APIC ID
    pub fn get_apic_id(&self) -> u32 {
        self.apic_id.load(Ordering::Acquire)
    }

    /// Identity is published together, before the AP is released, so a reader
    /// that sees one sees both.
    pub fn set_identity(&self, cpu_id: u32, apic_id: u32, stack_size: usize) {
        self.stack_size.store(stack_size, Ordering::Relaxed);
        self.cpu_id.store(cpu_id, Ordering::Relaxed);
        self.apic_id.store(apic_id, Ordering::Release);
    }

    /// Get NUMA node
    pub fn get_numa_node(&self) -> u32 {
        self.numa_node
    }

    /// Get stack base address
    pub fn get_stack_base(&self) -> u64 {
        self.stack_base.load(Ordering::Acquire)
    }

    /// Get stack size
    pub fn get_stack_size(&self) -> usize {
        self.stack_size.load(Ordering::Acquire)
    }

    /// Get idle cycles
    pub fn get_idle_cycles(&self) -> u64 {
        self.idle_cycles.load(Ordering::Relaxed)
    }

    /// Get total cycles
    pub fn get_total_cycles(&self) -> u64 {
        self.total_cycles.load(Ordering::Relaxed)
    }

    /// Get current PID
    pub fn get_current_pid(&self) -> u32 {
        self.current_pid.load(Ordering::Relaxed)
    }

    /// Get preempt disable count
    pub fn get_preempt_disable_count(&self) -> u32 {
        self.preempt_disable_count.load(Ordering::Relaxed)
    }

    /// Check if in interrupt
    pub fn is_in_interrupt(&self) -> bool {
        self.in_interrupt.load(Ordering::Relaxed)
    }

    /// Get last error
    pub fn get_last_error(&self) -> u32 {
        self.last_error.load(Ordering::Relaxed)
    }

    /// Calculate CPU utilization
    pub fn get_utilization(&self) -> f64 {
        let total = self.total_cycles.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let idle = self.idle_cycles.load(Ordering::Relaxed);
        1.0 - (idle as f64 / total as f64)
    }
}

#[derive(Debug)]
pub struct SmpStats {
    pub cpu_count: usize,
    pub cpus_online: usize,
    pub bsp_apic_id: u32,
    pub per_cpu: Vec<CpuStats>,
}

#[derive(Debug)]
pub struct CpuStats {
    pub cpu_id: u32,
    pub apic_id: u32,
    pub state: CpuState,
    pub idle_cycles: u64,
    pub total_cycles: u64,
    pub current_pid: u32,
}

impl CpuStats {
    /// Get CPU ID
    pub fn cpu_id(&self) -> u32 {
        self.cpu_id
    }

    /// Get APIC ID
    pub fn apic_id(&self) -> u32 {
        self.apic_id
    }

    /// Get CPU state
    pub fn state(&self) -> CpuState {
        self.state
    }

    /// Get idle cycles
    pub fn idle_cycles(&self) -> u64 {
        self.idle_cycles
    }

    /// Get total cycles
    pub fn total_cycles(&self) -> u64 {
        self.total_cycles
    }

    /// Get current PID
    pub fn current_pid(&self) -> u32 {
        self.current_pid
    }

    /// Calculate utilization
    pub fn utilization(&self) -> f64 {
        if self.total_cycles == 0 {
            return 0.0;
        }
        1.0 - (self.idle_cycles as f64 / self.total_cycles as f64)
    }
}

impl SmpStats {
    /// Get CPU count
    pub fn cpu_count(&self) -> usize {
        self.cpu_count
    }

    /// Get online CPU count
    pub fn cpus_online(&self) -> usize {
        self.cpus_online
    }

    /// Get BSP APIC ID
    pub fn bsp_apic_id(&self) -> u32 {
        self.bsp_apic_id
    }

    /// Get per-CPU stats
    pub fn per_cpu_stats(&self) -> &[CpuStats] {
        &self.per_cpu
    }
}
