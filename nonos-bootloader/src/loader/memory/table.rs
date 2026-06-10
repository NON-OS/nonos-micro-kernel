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

use super::record::AllocationRecord;
use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::memory;

pub struct AllocationTable {
    pub records: [AllocationRecord; memory::MAX_ALLOCATIONS],
    pub count: usize,
}

impl Default for AllocationTable {
    fn default() -> Self {
        Self::new()
    }
}

impl AllocationTable {
    pub const fn new() -> Self {
        Self {
            records: [AllocationRecord { address: 0, pages: 0, memory_type: 0 };
                memory::MAX_ALLOCATIONS],
            count: 0,
        }
    }

    pub fn record(&mut self, address: u64, pages: usize) -> LoaderResult<()> {
        if self.count >= memory::MAX_ALLOCATIONS {
            return Err(LoaderError::AllocationTableFull);
        }
        self.records[self.count] = AllocationRecord::new(address, pages);
        self.count += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn total_pages(&self) -> usize {
        self.records[..self.count].iter().map(|r| r.pages).sum()
    }
    pub fn total_bytes(&self) -> usize {
        self.total_pages() * memory::PAGE_SIZE
    }
    pub fn records(&self) -> &[AllocationRecord] {
        &self.records[..self.count]
    }
}
