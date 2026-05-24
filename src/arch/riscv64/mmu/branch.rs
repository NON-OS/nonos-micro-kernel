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

use super::state::{alloc_table, table_from_ppn};
use super::sv39::Sv39;
use super::table::PageTable;
use super::PAGE_SIZE;

pub unsafe fn child(table: &mut PageTable, index: usize) -> Option<&'static mut PageTable> {
    table.next_table_ppn(index).map(|ppn| table_from_ppn(ppn))
}

pub unsafe fn ensure_child(table: &mut PageTable, index: usize) -> Option<&'static mut PageTable> {
    if table.is_branch(index) {
        return child(table, index);
    }
    if table.is_leaf(index) {
        return None;
    }
    let next = alloc_table()?;
    table.set_branch(index, next.ppn());
    Some(next)
}

pub unsafe fn split_leaf(
    table: &mut PageTable,
    index: usize,
    level: usize,
) -> Option<&'static mut PageTable> {
    if !table.is_leaf(index) || level == 0 {
        return None;
    }
    let entry = table.entry(index);
    let flags = entry & 0xff;
    let base_ppn = Sv39::pte_ppn(entry);
    let child = alloc_table()?;
    let child_pages = (Sv39::block_size(level - 1) / PAGE_SIZE) as u64;
    for child_index in 0..512 {
        let ppn = base_ppn + (child_index as u64 * child_pages);
        child.set_entry(child_index, (ppn << 10) | flags);
    }
    table.set_branch(index, child.ppn());
    Some(child)
}
