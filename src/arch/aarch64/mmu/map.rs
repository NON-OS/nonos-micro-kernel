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

use super::state;
use super::{tlb, PageAttributes};

pub fn map_page(virt: u64, phys: u64, attrs: PageAttributes) {
    let index = PageIndex::new(virt);
    unsafe {
        if map_inner(index, phys, &attrs) {
            tlb::flush_tlb_page(virt);
        }
    }
}

unsafe fn map_inner(index: PageIndex, phys: u64, attrs: &PageAttributes) -> bool {
    if index.l0 >= 4 || index.l1 >= 512 || index.l1 >= 4 {
        return false;
    }
    if !state::l0().is_valid(index.l0) {
        state::l0().set_table(index.l0, state::l1_addr());
    }
    if !state::l1().is_valid(index.l1) {
        state::l1().set_table(index.l1, state::l2_addr(index.l1));
    }
    let l2 = state::l2(index.l1);
    if !l2.is_valid(index.l2) {
        l2.set_table(index.l2, state::l3_addr(index.l1, index.l2));
    }
    state::l3(index.l1, index.l2).set_page(index.l3, phys, attrs);
    true
}

pub(super) struct PageIndex {
    pub(super) l0: usize,
    pub(super) l1: usize,
    pub(super) l2: usize,
    pub(super) l3: usize,
}

impl PageIndex {
    pub(super) fn new(virt: u64) -> Self {
        Self {
            l0: ((virt >> 39) & 0x1FF) as usize,
            l1: ((virt >> 30) & 0x1FF) as usize,
            l2: ((virt >> 21) & 0x1FF) as usize,
            l3: ((virt >> 12) & 0x1FF) as usize,
        }
    }
}
