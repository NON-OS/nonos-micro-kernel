// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::table::boot::BootServices;

use crate::paging::constants::{HUGE_2M, PAGE_TABLE_ENTRIES, PTE_P, PTE_PS};
use crate::paging::table::PageTable;

use super::ensure_pd::ensure_pd;
use super::ensure_pdpt::ensure_pdpt;
use super::pd_index::pd_index;
use super::pdpt_index::pdpt_index;
use super::pml4_index::pml4_index;

// Map `count` consecutive 2-MiB-aligned virtual pages starting at
// `va_base` to the matching 2-MiB-aligned physical pages starting at
// `phys_base`, with the supplied flags. This is the universally
// supported fallback for `map_huge_1g_run` on CPUs without 1 GiB page
// support (see `paging::page1g`). Allocates one PDPT under each PML4
// slot and one PD under each PDPT slot the run touches; later runs
// into the same slots reuse those tables.
//
// Caller's `flags` must NOT include the address bits or the PS bit;
// this function adds `PTE_P | PTE_PS` and the phys address.
pub fn map_huge_2m_run(
    bs: &BootServices,
    pml4: PageTable,
    va_base: u64,
    phys_base: u64,
    count: usize,
    flags: u64,
) -> Result<(), &'static str> {
    if va_base & (HUGE_2M - 1) != 0 {
        return Err("map_huge_2m_run: VA not 2 MiB-aligned");
    }
    if phys_base & (HUGE_2M - 1) != 0 {
        return Err("map_huge_2m_run: phys not 2 MiB-aligned");
    }

    for i in 0..count {
        let va = va_base + (i as u64) * HUGE_2M;
        let phys = phys_base + (i as u64) * HUGE_2M;
        let pml4_idx = pml4_index(va);
        let pdpt_idx = pdpt_index(va);
        let pd_idx = pd_index(va);

        if pdpt_idx >= PAGE_TABLE_ENTRIES || pd_idx >= PAGE_TABLE_ENTRIES {
            return Err("map_huge_2m_run: table index overflow");
        }

        let pdpt_phys = ensure_pdpt(bs, pml4, pml4_idx, 0)?;
        let pdpt = PageTable::from_phys(pdpt_phys);
        let pd_phys = ensure_pd(bs, pdpt, pdpt_idx, 0)?;
        let pd = PageTable::from_phys(pd_phys);
        unsafe {
            pd.write_entry(pd_idx, phys | PTE_P | PTE_PS | flags);
        }
    }
    Ok(())
}
