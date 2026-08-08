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

//! How many CPUs there are and how they relate.
//!
//! Where that answer comes from is entirely per-architecture: CPUID and the
//! ACPI MADT on a PC, the device tree's `/cpus` node on an ARM board. What is
//! done with it afterwards is not, so the registry and every accessor live in
//! `state`.

mod state;

#[cfg(target_arch = "aarch64")]
mod probe_aarch64;
#[cfg(target_arch = "x86_64")]
mod probe_x86_64;

pub use state::{
    cpu_to_numa_node, cpus_share_cache, enumerate_cpus, get_ap_list, get_cpu_info, get_topology,
};

/// Probe the machine and record what it says. Returns the logical CPU count.
pub fn detect_cpus() -> usize {
    #[cfg(target_arch = "x86_64")]
    return probe_x86_64::detect();
    #[cfg(target_arch = "aarch64")]
    return probe_aarch64::detect();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return 1;
}
