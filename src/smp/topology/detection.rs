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
use spin::Mutex;

use super::types::{CpuInfo, CpuTopology};
use crate::smp::MAX_CPUS;

static TOPOLOGY: Mutex<Option<CpuTopology>> = Mutex::new(None);
static CPU_INFO: Mutex<Vec<CpuInfo>> = Mutex::new(Vec::new());
static AP_LIST: Mutex<Vec<u32>> = Mutex::new(Vec::new());

pub fn detect_cpus() -> usize {
    let mut topology = CpuTopology {
        logical_cpus: 1,
        physical_cores: 1,
        numa_nodes: 1,
        hyperthreading: false,
        x2apic: false,
    };

    let (max_basic, _vendor_ebx, _vendor_ecx, _vendor_edx) = cpuid(0, 0);

    let (_, _, ecx, edx) = cpuid(1, 0);
    topology.x2apic = (ecx & (1 << 21)) != 0;
    topology.hyperthreading = (edx & (1 << 28)) != 0;

    if max_basic >= 0x0B {
        topology.logical_cpus = detect_via_leaf_0b(&mut topology);
    } else if max_basic >= 0x04 {
        topology.logical_cpus = detect_via_leaf_04(&mut topology);
    } else if max_basic >= 0x01 {
        let (_, ebx, _, _) = cpuid(1, 0);
        topology.logical_cpus = ((ebx >> 16) & 0xFF) as usize;
        if topology.logical_cpus == 0 {
            topology.logical_cpus = 1;
        }
    }

    let acpi_enabled = crate::arch::x86_64::acpi::processors().iter().filter(|p| p.enabled).count();
    if acpi_enabled > 0 {
        topology.logical_cpus = acpi_enabled;
    }

    if topology.logical_cpus > MAX_CPUS {
        topology.logical_cpus = MAX_CPUS;
    }

    *TOPOLOGY.lock() = Some(topology);

    build_ap_list();

    topology.logical_cpus
}

fn detect_via_leaf_0b(topology: &mut CpuTopology) -> usize {
    let mut total_threads = 0;
    let mut total_cores = 0;

    for subleaf in 0..3 {
        let (_eax, ebx, ecx, _) = cpuid(0x0B, subleaf);

        let level_type = (ecx >> 8) & 0xFF;
        let num_procs = ebx & 0xFFFF;

        match level_type {
            0 => break,
            1 => {
                total_threads = num_procs as usize;
            }
            2 => {
                total_cores = num_procs as usize;
            }
            _ => {}
        }

        if level_type == 0 {
            break;
        }
    }

    if total_cores > 0 {
        topology.physical_cores = total_cores;
    }
    if total_threads > 0 {
        topology.hyperthreading = total_threads > total_cores;
        return total_threads;
    }

    1
}

fn detect_via_leaf_04(topology: &mut CpuTopology) -> usize {
    let (eax, _, _, _) = cpuid(4, 0);

    let max_cores = ((eax >> 26) & 0x3F) + 1;
    topology.physical_cores = max_cores as usize;

    let (_, ebx, _, _) = cpuid(1, 0);
    let logical = ((ebx >> 16) & 0xFF) as usize;

    if logical > 0 {
        topology.hyperthreading = logical > max_cores as usize;
        return logical;
    }

    max_cores as usize
}

fn build_ap_list() {
    let mut ap_list = AP_LIST.lock();
    ap_list.clear();

    let bsp_apic = crate::arch::x86_64::interrupt::apic::id();

    let procs = crate::arch::x86_64::acpi::processors();
    if !procs.is_empty() {
        for p in &procs {
            if p.enabled && p.apic_id != bsp_apic {
                ap_list.push(p.apic_id);
            }
        }
        return;
    }

    if let Some(topo) = *TOPOLOGY.lock() {
        for i in 0..topo.logical_cpus {
            let apic_id = i as u32;
            if apic_id != bsp_apic {
                ap_list.push(apic_id);
            }
        }
    }
}

pub fn get_ap_list() -> Vec<u32> {
    AP_LIST.lock().clone()
}

pub fn get_topology() -> Option<CpuTopology> {
    *TOPOLOGY.lock()
}

pub fn get_cpu_info(apic_id: u32) -> Option<CpuInfo> {
    CPU_INFO.lock().iter().find(|c| c.apic_id == apic_id).copied()
}

pub fn enumerate_cpus() -> Vec<u32> {
    let mut cpus = Vec::new();
    let bsp = crate::arch::x86_64::interrupt::apic::id();
    cpus.push(bsp);
    cpus.extend(AP_LIST.lock().iter().copied());
    cpus
}

pub fn cpu_to_numa_node(apic_id: u32) -> u32 {
    get_cpu_info(apic_id).map(|c| c.numa_node).unwrap_or(0)
}

pub fn cpus_share_cache(cpu1: u32, cpu2: u32, _level: u8) -> bool {
    if let (Some(info1), Some(info2)) = (get_cpu_info(cpu1), get_cpu_info(cpu2)) {
        info1.package_id == info2.package_id && info1.core_id == info2.core_id
    } else {
        false
    }
}

#[inline]
fn cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    let result = core::arch::x86_64::__cpuid_count(leaf, subleaf);
    (result.eax, result.ebx, result.ecx, result.edx)
}
