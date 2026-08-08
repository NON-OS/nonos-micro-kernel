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
        self.memory_map()
            .iter()
            .filter(|r| r.region_type == MemoryType::Available)
            .map(|r| r.size)
            .sum()
    }

    /// The regions filled so far.
    pub fn memory_map(&self) -> &[MemoryRegion] {
        &self.memory_regions[..self.memory_region_count]
    }

    /// Record a region. Silently drops anything past the fixed capacity, which
    /// is the honest answer here: this runs before the console and before the
    /// heap, so there is nowhere to report to and nothing to grow into.
    pub fn add_memory_region(&mut self, base: u64, size: u64, region_type: MemoryType) {
        if self.memory_region_count < self.memory_regions.len() {
            self.memory_regions[self.memory_region_count] =
                MemoryRegion { base, size, region_type };
            self.memory_region_count += 1;
        }
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
