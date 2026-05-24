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

use super::set_cbit_on_pte::set_cbit_on_pte;
use super::walk_pt::walk_pt;
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::paging::constants::{
    pte_address, pte_is_huge, pte_is_present, PAGE_TABLE_ENTRIES,
};

pub(super) unsafe fn walk_pd(pd_phys: u64, c_bit_mask: u64) -> u64 {
    let table_va = DIRECTMAP_BASE + (pd_phys & !c_bit_mask);
    let table = &mut *(table_va as *mut [u64; PAGE_TABLE_ENTRIES]);
    let mut touched = 0u64;
    for slot in table.iter_mut() {
        let value = *slot;
        if !pte_is_present(value) {
            continue;
        }
        if pte_is_huge(value) {
            if set_cbit_on_pte(slot, c_bit_mask) {
                touched += 1;
            }
            continue;
        }
        let child_phys = pte_address(value);
        touched += walk_pt(child_phys, c_bit_mask);
    }
    touched
}
