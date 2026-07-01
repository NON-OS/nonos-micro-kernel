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

//! Four-level x86-64 page-table walker. Reads every level through
//! the directmap so the user virtual address itself is never
//! dereferenced. Honours 1 GiB and 2 MiB huge pages.

use super::bounds::{directmap_of, leaf_in_directmap};
use super::leaf::UserLeaf;
use super::root::page_table_root;
use crate::memory::paging::constants::{PTE_ADDR_MASK, PTE_HUGE_PAGE, PTE_PRESENT, PTE_USER};
use crate::usercopy::error::UsercopyError;

const PAGE_TABLE_INDEX_MASK: u64 = 0x1FF;
const PAGE_4K_MASK: u64 = 0xFFF;
const PAGE_2M_MASK: u64 = (1 << 21) - 1;
const PAGE_1G_MASK: u64 = (1 << 30) - 1;
const PAGE_4K_SIZE: u64 = 1 << 12;
const PAGE_2M_SIZE: u64 = 1 << 21;
const PAGE_1G_SIZE: u64 = 1 << 30;
const PAGE_2M_ADDR_MASK: u64 = 0x000F_FFFF_FFE0_0000;
const PAGE_1G_ADDR_MASK: u64 = 0x000F_FFFF_C000_0000;

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
    if e4 & PTE_PRESENT == 0 {
        return Err(UsercopyError::PageNotMapped);
    }
    if e4 & PTE_USER == 0 {
        return Err(UsercopyError::PageNotUser);
    }
    let e3 = read_pte(directmap_of(e4 & PTE_ADDR_MASK)?, i3);
    if e3 & PTE_PRESENT == 0 {
        return Err(UsercopyError::PageNotMapped);
    }
    if e3 & PTE_USER == 0 {
        return Err(UsercopyError::PageNotUser);
    }
    if e3 & PTE_HUGE_PAGE != 0 {
        return leaf_in_directmap(UserLeaf {
            entry: e3,
            phys_base: e3 & PAGE_1G_ADDR_MASK,
            offset: va & PAGE_1G_MASK,
            size: PAGE_1G_SIZE,
        });
    }
    let e2 = read_pte(directmap_of(e3 & PTE_ADDR_MASK)?, i2);
    if e2 & PTE_PRESENT == 0 {
        return Err(UsercopyError::PageNotMapped);
    }
    if e2 & PTE_USER == 0 {
        return Err(UsercopyError::PageNotUser);
    }
    if e2 & PTE_HUGE_PAGE != 0 {
        return leaf_in_directmap(UserLeaf {
            entry: e2,
            phys_base: e2 & PAGE_2M_ADDR_MASK,
            offset: va & PAGE_2M_MASK,
            size: PAGE_2M_SIZE,
        });
    }
    let e1 = read_pte(directmap_of(e2 & PTE_ADDR_MASK)?, i1);
    if e1 & PTE_PRESENT == 0 {
        return Err(UsercopyError::PageNotMapped);
    }
    leaf_in_directmap(UserLeaf {
        entry: e1,
        phys_base: e1 & PTE_ADDR_MASK,
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
