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

//! CPU topology from the device tree.
//!
//! There is no CPUID to interrogate and no MADT to walk. The `/cpus` node is
//! the whole answer, and the boot path already latched it into the roster, so
//! this reads that rather than parsing again.
//!
//! Package and core structure would come from `cpu-map`, which this does not
//! parse yet. Reporting one core per CPU and no hyperthreading is what the
//! device tree actually told us; inventing a hierarchy would give the
//! scheduler cache-sharing hints with nothing behind them.

extern crate alloc;

use alloc::vec::Vec;

use super::super::types::CpuTopology;
use super::state::{set_ap_list, set_topology};
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::cpu::pack_affinity;
use crate::smp::MAX_CPUS;

pub(super) fn detect() -> usize {
    let count = roster::len().min(MAX_CPUS).max(1);

    set_topology(CpuTopology {
        logical_cpus: count,
        physical_cores: count,
        numa_nodes: 1,
        hyperthreading: false,
        x2apic: false,
    });

    set_ap_list(secondaries());
    count
}

/// Every CPU in the roster except the one running, named by the packed
/// affinity the GIC addresses it with.
fn secondaries() -> Vec<u32> {
    let own = crate::arch::interrupt_controller::local_id();
    (0..roster::len().min(MAX_CPUS))
        .filter_map(roster::affinity_of)
        .map(pack_affinity)
        .filter(|affinity| *affinity != own)
        .collect()
}
