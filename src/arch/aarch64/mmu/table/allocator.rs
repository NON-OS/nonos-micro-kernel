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

use core::ptr::NonNull;

use super::page_table::PageTable;

pub struct PageTableAllocator {
    next_table: *mut PageTable,
    end: *mut PageTable,
}

impl PageTableAllocator {
    pub fn new(start: u64, size: usize) -> Self {
        let count = size / core::mem::size_of::<PageTable>();
        Self {
            next_table: start as *mut PageTable,
            end: (start as *mut PageTable).wrapping_add(count),
        }
    }

    pub fn alloc(&mut self) -> Option<NonNull<PageTable>> {
        if self.next_table >= self.end {
            return None;
        }
        let table = self.next_table;
        self.next_table = self.next_table.wrapping_add(1);
        unsafe {
            core::ptr::write_bytes(table, 0, 1);
        }
        NonNull::new(table)
    }

    pub fn remaining(&self) -> usize {
        (self.end as usize - self.next_table as usize) / core::mem::size_of::<PageTable>()
    }
}
