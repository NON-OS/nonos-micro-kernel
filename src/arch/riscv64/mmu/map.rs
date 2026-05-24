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

use super::branch::{ensure_child, split_leaf};
use super::state::root_table;
use super::sv39::Sv39;
use super::{tlb, PageAttributes};

pub fn map_page(virt: u64, phys: u64, attrs: PageAttributes) {
    let va = virt as usize;
    unsafe {
        if map_page_inner(va, phys, &attrs) {
            tlb::flush_tlb_page(va);
        }
    }
}

unsafe fn map_page_inner(va: usize, phys: u64, attrs: &PageAttributes) -> bool {
    let root = root_table();
    let vpn2 = Sv39::vpn(va, 2);
    let vpn1 = Sv39::vpn(va, 1);
    let vpn0 = Sv39::vpn(va, 0);
    if root.is_leaf(vpn2) && split_leaf(root, vpn2, 2).is_none() {
        return false;
    }
    let Some(l1) = ensure_child(root, vpn2) else {
        return false;
    };
    if l1.is_leaf(vpn1) && split_leaf(l1, vpn1, 1).is_none() {
        return false;
    }
    let Some(l0) = ensure_child(l1, vpn1) else {
        return false;
    };
    l0.set_leaf(vpn0, phys >> 12, attrs);
    true
}
