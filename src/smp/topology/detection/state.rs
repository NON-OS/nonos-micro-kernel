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

//! What the probe found, and the questions the rest of the kernel asks of it.
//! Neither depends on how it was found.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::super::types::{CpuInfo, CpuTopology};

static TOPOLOGY: Mutex<Option<CpuTopology>> = Mutex::new(None);
static CPU_INFO: Mutex<Vec<CpuInfo>> = Mutex::new(Vec::new());
static AP_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());

pub(super) fn set_topology(topology: CpuTopology) {
    *TOPOLOGY.lock() = Some(topology);
}

pub(super) fn set_ap_list(ids: Vec<u32>) {
    *AP_LIST.lock() = ids;
}

pub fn get_topology() -> Option<CpuTopology> {
    *TOPOLOGY.lock()
}

/// Every secondary the platform reported, named as the interrupt controller
/// names it.
pub fn get_ap_list() -> Vec<u32> {
    AP_LIST.lock().clone()
}

pub fn get_cpu_info(id: u32) -> Option<CpuInfo> {
    CPU_INFO.lock().iter().find(|c| c.apic_id == id).copied()
}

/// The boot CPU followed by every secondary.
pub fn enumerate_cpus() -> Vec<u32> {
    let mut cpus = Vec::new();
    cpus.push(crate::arch::interrupt_controller::local_id());
    cpus.extend(AP_LIST.lock().iter().copied());
    cpus
}

pub fn cpu_to_numa_node(id: u32) -> u32 {
    get_cpu_info(id).map(|c| c.numa_node).unwrap_or(0)
}

pub fn cpus_share_cache(first: u32, second: u32, _level: u8) -> bool {
    match (get_cpu_info(first), get_cpu_info(second)) {
        (Some(a), Some(b)) => a.package_id == b.package_id && a.core_id == b.core_id,
        _ => false,
    }
}
