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

//! Four-level page-table walker. Reads every level through the directmap so
//! the user virtual address itself is never dereferenced. Honours 1 GiB and
//! 2 MiB huge pages.
//!
//! Both supported architectures index four levels of nine bits over a 4 KiB
//! page, so the shape of the walk is shared. What an entry means is not, and
//! every test here goes through `arch::paging::descriptor`: reading the wrong
//! bit in the walk that decides whether userspace may touch an address is a
//! privilege bug, not a portability nit.
//!
//! One asymmetry the boundary hides is worth knowing about. x86_64 intersects
//! permissions down the hierarchy, so a table entry without the user bit
//! denies EL0 whatever the leaf says. An aarch64 table descriptor restricts
//! nothing unless its hierarchical bits say so. `table_grants_user` answers
//! that per architecture; `is_user` answers it for the leaf.

use super::bounds::{directmap_of, leaf_in_directmap};
use super::leaf::UserLeaf;
use super::root::page_table_root;
use crate::arch::paging::descriptor;
use crate::usercopy::error::UsercopyError;

const PAGE_TABLE_INDEX_MASK: u64 = 0x1FF;
const PAGE_4K_MASK: u64 = 0xFFF;
const PAGE_2M_MASK: u64 = (1 << 21) - 1;
const PAGE_1G_MASK: u64 = (1 << 30) - 1;
const PAGE_4K_SIZE: u64 = 1 << 12;
const PAGE_2M_SIZE: u64 = 1 << 21;
const PAGE_1G_SIZE: u64 = 1 << 30;
/// A block descriptor's output address is the page-address bits with the bits
/// the block spans cleared, so both are derived from the architecture's own
/// address mask rather than written out.
const PAGE_2M_ADDR_MASK: u64 = descriptor::ADDR_MASK & !PAGE_2M_MASK;
const PAGE_1G_ADDR_MASK: u64 = descriptor::ADDR_MASK & !PAGE_1G_MASK;

// Internal entry: page-table walk only. Permission-aware callers
// live in `access.rs` and are the only translators the rest of the
// usercopy tree may use; this routine has no `pub(crate)` exposure.
pub(super) fn walk_to_leaf(va: u64) -> Result<UserLeaf, UsercopyError> {
    let pt_root = page_table_root()?;
    if pt_root == 0 || pt_root & PAGE_4K_MASK != 0 {
        return Err(UsercopyError::PageTableCorrupt);
    }
    walk(pt_root, va)
}

fn walk(pt_root: u64, va: u64) -> Result<UserLeaf, UsercopyError> {
    let i4 = (va >> 39) & PAGE_TABLE_INDEX_MASK;
    let i3 = (va >> 30) & PAGE_TABLE_INDEX_MASK;
    let i2 = (va >> 21) & PAGE_TABLE_INDEX_MASK;
    let i1 = (va >> 12) & PAGE_TABLE_INDEX_MASK;

    let e4 = read_pte(directmap_of(pt_root)?, i4);
    if !descriptor::is_present(e4) {
        return Err(UsercopyError::PageNotMapped);
    }
    if !descriptor::table_grants_user(e4) {
        return Err(UsercopyError::PageNotUser);
    }
    let e3 = read_pte(directmap_of(descriptor::address(e4))?, i3);
    if !descriptor::is_present(e3) {
        return Err(UsercopyError::PageNotMapped);
    }
    if descriptor::is_block(e3) {
        // Permission on the leaf itself belongs to `access.rs`, which is the
        // only caller and checks it on the entry returned here.
        return leaf_in_directmap(UserLeaf {
            entry: e3,
            phys_base: e3 & PAGE_1G_ADDR_MASK,
            offset: va & PAGE_1G_MASK,
            size: PAGE_1G_SIZE,
        });
    }
    if !descriptor::table_grants_user(e3) {
        return Err(UsercopyError::PageNotUser);
    }
    let e2 = read_pte(directmap_of(descriptor::address(e3))?, i2);
    if !descriptor::is_present(e2) {
        return Err(UsercopyError::PageNotMapped);
    }
    if descriptor::is_block(e2) {
        return leaf_in_directmap(UserLeaf {
            entry: e2,
            phys_base: e2 & PAGE_2M_ADDR_MASK,
            offset: va & PAGE_2M_MASK,
            size: PAGE_2M_SIZE,
        });
    }
    if !descriptor::table_grants_user(e2) {
        return Err(UsercopyError::PageNotUser);
    }
    let e1 = read_pte(directmap_of(descriptor::address(e2))?, i1);
    if !descriptor::is_present(e1) {
        return Err(UsercopyError::PageNotMapped);
    }
    leaf_in_directmap(UserLeaf {
        entry: e1,
        phys_base: descriptor::address(e1),
        offset: va & PAGE_4K_MASK,
        size: PAGE_4K_SIZE,
    })
}

fn read_pte(table_virt: u64, index: u64) -> u64 {
    let ptr = (table_virt + index * 8) as *const u64;
    // SAFETY: ek@nonos.systems — `table_virt` is a directmap VA derived
    // from a present PTE. `index` is masked to 0..=511 so the offset
    // stays inside the 4 KiB table page.
    unsafe { core::ptr::read_volatile(ptr) }
}
