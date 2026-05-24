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

use alloc::vec::Vec;

use super::memory::MemoryRegion;

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub ram_base: u64,
    pub ram_size: u64,
    pub kernel_base: u64,
    pub kernel_size: u64,
    pub dtb_base: u64,
    pub dtb_size: u64,
    pub uart_base: u64,
    pub gic_dist_base: u64,
    pub gic_redist_base: u64,
    pub cpu_count: u32,
    pub timer_phys_intid: u32,
    pub timer_virt_intid: u32,
    pub gic_unsupported: bool,
    pub memory_regions: Vec<MemoryRegion>,
}

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            ram_base: 0x4000_0000,
            ram_size: 0x1_0000_0000,
            kernel_base: 0x4000_0000,
            kernel_size: 0x0020_0000,
            dtb_base: 0,
            dtb_size: 0,
            uart_base: 0x0900_0000,
            gic_dist_base: 0x0800_0000,
            gic_redist_base: 0x080A_0000,
            cpu_count: 1,
            timer_phys_intid: 0,
            timer_virt_intid: 0,
            gic_unsupported: false,
            memory_regions: Vec::new(),
        }
    }
}
