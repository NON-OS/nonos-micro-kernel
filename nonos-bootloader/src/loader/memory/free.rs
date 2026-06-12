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

use super::table::AllocationTable;
use crate::loader::types::memory;

pub fn to_array(table: &AllocationTable) -> ([(u64, usize); memory::MAX_ALLOCATIONS], usize) {
    let mut arr = [(0u64, 0usize); memory::MAX_ALLOCATIONS];
    for (i, record) in table.records[..table.count].iter().enumerate() {
        arr[i] = (record.address, record.pages);
    }
    (arr, table.count)
}

pub fn free_all(table: &AllocationTable, bs: &uefi::table::boot::BootServices) {
    for record in &table.records[..table.count] {
        if record.is_valid() {
            let _ = bs.free_pages(record.address, record.pages);
        }
    }
}
