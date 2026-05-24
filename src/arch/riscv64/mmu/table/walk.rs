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

use super::page_table::PageTable;
use super::super::attributes::PteFlags;
use super::super::sv39::Sv39;

pub fn walk_page_tables(root: &PageTable, va: usize) -> Option<(u64, usize)> {
    let mut table = root;
    for level in (0..3).rev() {
        let index = Sv39::vpn(va, level);
        let entry = table.entry(index);
        if entry & PteFlags::V == 0 {
            return None;
        }
        if entry & (PteFlags::R | PteFlags::W | PteFlags::X) != 0 {
            return Some((Sv39::pte_ppn(entry), level));
        }
        table = unsafe { &*((Sv39::pte_ppn(entry) << 12) as *const PageTable) };
    }
    None
}
