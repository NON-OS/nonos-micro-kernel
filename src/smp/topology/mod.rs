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

pub mod detection;
pub mod types;

pub use types::{CpuInfo, CpuTopology, MAX_NUMA_NODES};

pub use detection::{
    cpu_to_numa_node, cpus_share_cache, detect_cpus, enumerate_cpus, get_ap_list, get_cpu_info,
    get_topology,
};
