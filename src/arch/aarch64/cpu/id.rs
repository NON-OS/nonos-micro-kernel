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

mod affinity;
mod info;
mod main_id;
mod mpidr;

pub use affinity::{
    affinity_level, cluster_id, core_id, cpu_affinity, cpu_id, is_multiprocessor, is_primary_core,
    mpidr_affinity, pack_affinity,
};
pub use info::CpuInfo;
pub use main_id::{architecture, implementer, main_id, part_number, revision, variant};
pub use mpidr::mpidr;
