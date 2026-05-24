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

use super::memory::{MemoryRegion, MemoryType};
use super::types::BootInfo;

impl BootInfo {
    pub fn total_memory(&self) -> u64 {
        self.memory_regions
            .iter()
            .filter(|r| r.region_type == MemoryType::Available)
            .map(|r| r.size)
            .sum()
    }

    pub fn add_memory_region(&mut self, base: u64, size: u64, region_type: MemoryType) {
        self.memory_regions.push(MemoryRegion { base, size, region_type });
    }

    pub fn usable_memory_start(&self) -> u64 {
        self.kernel_base + self.kernel_size
    }

    pub fn usable_memory_size(&self) -> u64 {
        let end = self.ram_base + self.ram_size;
        let start = self.usable_memory_start();
        end.saturating_sub(start)
    }
}
