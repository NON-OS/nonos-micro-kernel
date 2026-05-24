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

use super::walk_pdpt::walk_pdpt;
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::paging::constants::{pte_address, pte_is_present, PAGE_TABLE_ENTRIES};

pub(super) unsafe fn walk_pml4_entry(pml4_phys: u64, index: usize, c_bit_mask: u64) -> u64 {
    if index >= PAGE_TABLE_ENTRIES {
        return 0;
    }
    let table_va = DIRECTMAP_BASE + (pml4_phys & !c_bit_mask);
    let table = &*(table_va as *const [u64; PAGE_TABLE_ENTRIES]);
    let entry = table[index];
    if !pte_is_present(entry) {
        return 0;
    }
    let pdpt_phys = pte_address(entry);
    walk_pdpt(pdpt_phys, c_bit_mask)
}
