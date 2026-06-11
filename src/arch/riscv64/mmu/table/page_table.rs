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

use super::super::attributes::{PageAttributes, PteFlags};
use super::super::sv39::Sv39;

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u64; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        Self { entries: [0; 512] }
    }
    pub fn entry(&self, index: usize) -> u64 {
        self.entries[index]
    }
    pub fn set_entry(&mut self, index: usize, entry: u64) {
        self.entries[index] = entry;
    }
    pub fn clear_entry(&mut self, index: usize) {
        self.entries[index] = 0;
    }
    pub fn is_valid(&self, index: usize) -> bool {
        self.entries[index] & PteFlags::V != 0
    }
    pub fn is_leaf(&self, index: usize) -> bool {
        is_leaf_entry(self.entries[index])
    }
    pub fn is_branch(&self, index: usize) -> bool {
        is_branch_entry(self.entries[index])
    }

    pub fn next_table_ppn(&self, index: usize) -> Option<u64> {
        self.is_branch(index).then(|| Sv39::pte_ppn(self.entries[index]))
    }

    pub fn page_ppn(&self, index: usize) -> Option<u64> {
        self.is_leaf(index).then(|| Sv39::pte_ppn(self.entries[index]))
    }

    pub fn set_branch(&mut self, index: usize, table_ppn: u64) {
        self.entries[index] = Sv39::make_pte(table_ppn, PteFlags::new().valid());
    }

    pub fn set_leaf(&mut self, index: usize, phys_ppn: u64, attrs: &PageAttributes) {
        self.entries[index] = Sv39::make_pte(phys_ppn, attrs.to_pte_flags());
    }

    pub fn as_ptr(&self) -> *const u64 {
        self.entries.as_ptr()
    }
    pub fn as_mut_ptr(&mut self) -> *mut u64 {
        self.entries.as_mut_ptr()
    }
    pub fn physical_address(&self) -> u64 {
        self.entries.as_ptr() as u64
    }
    pub fn ppn(&self) -> u64 {
        self.physical_address() >> 12
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

fn is_leaf_entry(entry: u64) -> bool {
    (entry & PteFlags::V != 0) && (entry & (PteFlags::R | PteFlags::W | PteFlags::X) != 0)
}

fn is_branch_entry(entry: u64) -> bool {
    (entry & PteFlags::V != 0) && (entry & (PteFlags::R | PteFlags::W | PteFlags::X) == 0)
}
