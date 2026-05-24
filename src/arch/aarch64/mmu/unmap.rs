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

use super::map::PageIndex;
use super::{state, tlb};

pub fn unmap_page(virt: u64) {
    let index = PageIndex::new(virt);
    unsafe {
        if unmap_inner(index) {
            tlb::flush_tlb_page(virt);
        }
    }
}

unsafe fn unmap_inner(index: PageIndex) -> bool {
    if index.l0 >= 4 || index.l1 >= 4 {
        return false;
    }
    if !state::l0().is_valid(index.l0) || !state::l1().is_valid(index.l1) {
        return false;
    }
    let l2 = state::l2(index.l1);
    if !l2.is_valid(index.l2) {
        return false;
    }
    if l2.is_block(index.l2) {
        l2.clear_entry(index.l2);
    } else {
        state::l3(index.l1, index.l2).clear_entry(index.l3);
    }
    true
}
