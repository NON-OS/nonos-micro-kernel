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

use super::memory::MemoryRegion;

/// Enough for the memory nodes a board actually publishes. Fixed because the
/// device tree is walked before the heap exists.
pub const MAX_MEMORY_REGIONS: usize = 8;

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub ram_base: u64,
    pub ram_size: u64,
    pub kernel_base: u64,
    pub kernel_size: u64,
    pub dtb_base: u64,
    pub dtb_size: u64,
    pub uart_base: u64,
    pub plic_base: u64,
    pub clint_base: u64,
    pub hart_count: u32,
    pub boot_hart: u32,
    /// Fixed storage: this is filled from the device tree in the entry path,
    /// long before the heap exists, so it cannot be a `Vec`.
    pub memory_regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    pub memory_region_count: usize,
}

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            ram_base: 0x8000_0000,
            ram_size: 0x1_0000_0000,
            kernel_base: 0x8020_0000,
            kernel_size: 0x0020_0000,
            dtb_base: 0,
            dtb_size: 0,
            uart_base: 0x1000_0000,
            plic_base: 0,
            clint_base: 0x0200_0000,
            hart_count: 1,
            boot_hart: 0,
            memory_regions: [MemoryRegion::EMPTY; MAX_MEMORY_REGIONS],
            memory_region_count: 0,
        }
    }
}
