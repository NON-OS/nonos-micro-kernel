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

use super::affinity::{cluster_id, core_id, cpu_id};
use super::main_id::{implementer, part_number, revision, variant};

#[derive(Debug, Clone, Copy)]
pub struct CpuInfo {
    pub cpu_id: usize,
    pub core_id: usize,
    pub cluster_id: usize,
    pub implementer: u8,
    pub part_number: u16,
    pub variant: u8,
    pub revision: u8,
}

impl CpuInfo {
    pub fn current() -> Self {
        Self {
            cpu_id: cpu_id(),
            core_id: core_id(),
            cluster_id: cluster_id(),
            implementer: implementer(),
            part_number: part_number(),
            variant: variant(),
            revision: revision(),
        }
    }
}
