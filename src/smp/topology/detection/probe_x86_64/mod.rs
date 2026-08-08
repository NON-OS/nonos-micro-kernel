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

//! CPU topology from CPUID, corrected by the ACPI MADT.
//!
//! CPUID describes what one package can do; the MADT lists what the firmware
//! actually enabled, which is the smaller and truer number on a machine with
//! cores disabled or a socket unpopulated.

extern crate alloc;

mod leaves;

use alloc::vec::Vec;

use super::super::types::CpuTopology;
use super::state::{set_ap_list, set_topology};
use crate::smp::MAX_CPUS;
use leaves::{cpuid, via_leaf_04, via_leaf_0b};

pub(super) fn detect() -> usize {
    let mut topology = CpuTopology {
        logical_cpus: 1,
        physical_cores: 1,
        numa_nodes: 1,
        hyperthreading: false,
        x2apic: false,
    };

    let (max_basic, ..) = cpuid(0, 0);
    let (_, _, ecx, edx) = cpuid(1, 0);
    topology.x2apic = ecx & (1 << 21) != 0;
    topology.hyperthreading = edx & (1 << 28) != 0;

    topology.logical_cpus = if max_basic >= 0x0B {
        via_leaf_0b(&mut topology)
    } else if max_basic >= 0x04 {
        via_leaf_04(&mut topology)
    } else if max_basic >= 0x01 {
        (((cpuid(1, 0).1) >> 16) & 0xFF).max(1) as usize
    } else {
        1
    };

    let enabled = crate::arch::x86_64::acpi::processors().iter().filter(|p| p.enabled).count();
    if enabled > 0 {
        topology.logical_cpus = enabled;
    }
    topology.logical_cpus = topology.logical_cpus.min(MAX_CPUS);

    set_topology(topology);
    set_ap_list(secondaries(topology.logical_cpus));
    topology.logical_cpus
}

/// Every enabled processor the MADT lists except this one. Without a MADT the
/// APIC IDs are assumed dense from zero, which is what a machine old enough to
/// lack one does.
fn secondaries(logical_cpus: usize) -> Vec<u32> {
    let own = crate::arch::interrupt_controller::local_id();
    let processors = crate::arch::x86_64::acpi::processors();
    if !processors.is_empty() {
        return processors
            .iter()
            .filter(|p| p.enabled && p.apic_id != own)
            .map(|p| p.apic_id)
            .collect();
    }
    (0..logical_cpus as u32).filter(|id| *id != own).collect()
}
