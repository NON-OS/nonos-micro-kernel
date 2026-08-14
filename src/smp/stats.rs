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

use super::state::{cpu_count, cpus_online, BSP_APIC_ID, CPU_DESCRIPTORS};
use super::types::{CpuStats, SmpStats};
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

pub fn get_smp_stats() -> SmpStats {
    let mut per_cpu = Vec::new();

    for i in 0..cpu_count() {
        let cpu = &CPU_DESCRIPTORS[i];
        per_cpu.push(CpuStats {
            cpu_id: cpu.get_cpu_id(),
            apic_id: cpu.get_apic_id(),
            state: cpu.state(),
            idle_cycles: cpu.idle_cycles.load(Ordering::Relaxed),
            total_cycles: cpu.total_cycles.load(Ordering::Relaxed),
            current_pid: cpu.current_pid.load(Ordering::Relaxed),
        });
    }

    SmpStats {
        cpu_count: cpu_count(),
        cpus_online: cpus_online(),
        bsp_apic_id: BSP_APIC_ID.load(Ordering::Relaxed),
        per_cpu,
    }
}
