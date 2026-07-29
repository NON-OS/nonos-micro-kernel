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

//! Walk a dying address space's PML4[0..256] (user half) and free
//! every leaf frame plus every PDPT/PD/PT subtable allocated under
//! it. PML4[256..511] entries point at PDPTs shared by all live
//! address spaces (seeded once from KERNEL_ASID); they are not
//! touched here.

use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc;
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::paging::constants::{
    pte_address, pte_is_huge, pte_is_present, PAGE_TABLE_ENTRIES,
};

const KERNEL_HALF_START: usize = PAGE_TABLE_ENTRIES / 2;

const _: () = assert!(
    KERNEL_HALF_START == 256,
    "PML4[0..256] is the user half; cleanup must never iterate past 256",
);

fn table_at(phys: u64) -> *mut [u64; PAGE_TABLE_ENTRIES] {
    (DIRECTMAP_BASE + phys) as *mut [u64; PAGE_TABLE_ENTRIES]
}

// SAFETY: ek@nonos.systems — `table_phys` was the address of a
// 4 KiB page-table page allocated by this manager and unreachable
// from any active CR3 by the time `teardown_user_half` runs.
unsafe fn free_subtree(table_phys: u64, level: u8) {
    let table = unsafe { &mut *table_at(table_phys) };
    for entry in table.iter_mut() {
        let value = *entry;
        if !pte_is_present(value) {
            continue;
        }
        let next_phys = pte_address(value);
        if level == 1 || pte_is_huge(value) {
            let _ = frame_alloc::deallocate_frame(PhysAddr::new(next_phys));
        } else {
            unsafe { free_subtree(next_phys, level - 1) };
            let _ = frame_alloc::deallocate_frame(PhysAddr::new(next_phys));
        }
        *entry = 0;
    }
}

pub(super) fn teardown_user_half(cr3_value: PhysAddr) {
    // SAFETY: same precondition as `free_subtree`. The PML4 frame
    // is owned by this manager.
    unsafe {
        let pml4 = &mut *table_at(cr3_value.as_u64());
        for i in 0..KERNEL_HALF_START {
            let value = pml4[i];
            if !pte_is_present(value) {
                continue;
            }
            let pdpt_phys = pte_address(value);
            free_subtree(pdpt_phys, 3);
            let _ = frame_alloc::deallocate_frame(PhysAddr::new(pdpt_phys));
            pml4[i] = 0;
        }
    }
}
