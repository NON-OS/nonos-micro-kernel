// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::loader::types::memory;
use uefi::table::boot::MemoryType;

#[derive(Debug, Clone, Copy, Default)]
pub struct AllocationRecord {
    pub address: u64,
    pub pages: usize,
    pub memory_type: u32,
}

impl AllocationRecord {
    pub fn new(address: u64, pages: usize) -> Self {
        Self { address, pages, memory_type: MemoryType::LOADER_DATA.0 }
    }
    pub fn is_valid(&self) -> bool {
        self.address != 0 && self.pages > 0
    }
    pub fn size_bytes(&self) -> usize {
        self.pages * memory::PAGE_SIZE
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: usize,
    pub writable: bool,
    pub executable: bool,
}

impl MemoryRegion {
    pub fn end(&self) -> u64 {
        self.start + self.size as u64
    }
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.end()
    }
    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}
