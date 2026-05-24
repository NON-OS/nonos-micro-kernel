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

use super::branch::{child, split_leaf};
use super::state::root_table;
use super::sv39::Sv39;
use super::table::PageTable;
use super::tlb;

pub fn unmap_page(virt: u64) {
    let va = virt as usize;
    unsafe {
        if unmap_from(root_table(), va, 2) {
            tlb::flush_tlb_page(va);
        }
    }
}

unsafe fn unmap_from(table: &mut PageTable, va: usize, level: usize) -> bool {
    let index = Sv39::vpn(va, level);
    if !table.is_valid(index) {
        return false;
    }
    if table.is_leaf(index) {
        return unmap_leaf(table, index, va, level);
    }
    match child(table, index) {
        Some(next) => unmap_from(next, va, level - 1),
        None => false,
    }
}

unsafe fn unmap_leaf(table: &mut PageTable, index: usize, va: usize, level: usize) -> bool {
    if level == 0 {
        table.clear_entry(index);
        return true;
    }
    match split_leaf(table, index, level) {
        Some(next) => unmap_from(next, va, level - 1),
        None => false,
    }
}
