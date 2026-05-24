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

use super::table::PageTable;

const TABLE_POOL_COUNT: usize = 256;

static mut ROOT_TABLE: PageTable = PageTable::new();
static mut TABLE_POOL: [PageTable; TABLE_POOL_COUNT] = [const { PageTable::new() }; TABLE_POOL_COUNT];
static mut NEXT_TABLE: usize = 0;

pub unsafe fn root_table() -> &'static mut PageTable {
    &mut ROOT_TABLE
}

pub unsafe fn alloc_table() -> Option<&'static mut PageTable> {
    let index = NEXT_TABLE;
    if index == TABLE_POOL_COUNT {
        return None;
    }
    NEXT_TABLE = index + 1;
    TABLE_POOL[index] = PageTable::new();
    Some(&mut TABLE_POOL[index])
}

pub unsafe fn table_from_ppn(ppn: u64) -> &'static mut PageTable {
    &mut *((ppn << 12) as *mut PageTable)
}
