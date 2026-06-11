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

use super::super::attributes::{PageAttributes, PTE_ADDR_MASK, PTE_PAGE, PTE_TABLE, PTE_VALID};
use super::page_table::PageTable;

impl PageTable {
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
        self.entries[index] & PTE_VALID != 0
    }

    pub fn is_table(&self, index: usize) -> bool {
        let entry = self.entries[index];
        (entry & PTE_VALID != 0) && (entry & PTE_TABLE != 0)
    }

    pub fn is_block(&self, index: usize) -> bool {
        let entry = self.entries[index];
        (entry & PTE_VALID != 0) && (entry & PTE_TABLE == 0)
    }

    pub fn table_address(&self, index: usize) -> Option<u64> {
        self.is_table(index).then_some(self.entries[index] & PTE_ADDR_MASK)
    }

    pub fn block_address(&self, index: usize) -> Option<u64> {
        self.is_block(index).then_some(self.entries[index] & PTE_ADDR_MASK)
    }

    pub fn set_table(&mut self, index: usize, table_addr: u64) {
        self.entries[index] = (table_addr & PTE_ADDR_MASK) | PTE_TABLE | PTE_VALID;
    }

    pub fn set_block(&mut self, index: usize, phys_addr: u64, attrs: &PageAttributes) {
        self.entries[index] = (phys_addr & PTE_ADDR_MASK) | attrs.to_descriptor_bits() | PTE_VALID;
    }

    pub fn set_page(&mut self, index: usize, phys_addr: u64, attrs: &PageAttributes) {
        self.entries[index] =
            (phys_addr & PTE_ADDR_MASK) | attrs.to_descriptor_bits() | PTE_PAGE | PTE_VALID;
    }
}
