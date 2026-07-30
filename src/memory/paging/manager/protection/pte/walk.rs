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

use super::super::super::core::PagingManager;
use super::super::super::shootdown::flush_tlb_one_smp;
use super::super::super::tlb_scope::mutation_asid;
use crate::arch::paging::descriptor;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::layout;
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};

impl PagingManager {
    pub(in super::super) fn update_pte(&self, va: VirtAddr, new_flags: u64) -> PagingResult<()> {
        let va_val = va.as_u64();
        let l4_idx = pml4_index(va_val);
        let l3_idx = pdpt_index(va_val);
        let l2_idx = pd_index(va_val);
        let l1_idx = pt_index(va_val);
        let cr3 = self.active_page_table.ok_or(PagingError::NoActivePageTable)?;
        unsafe {
            let l4_table =
                &*((layout::DIRECTMAP_BASE + cr3.as_u64()) as *const [u64; PAGE_TABLE_ENTRIES]);
            if !pte_is_present(l4_table[l4_idx]) {
                return Err(PagingError::Pml4NotPresent);
            }
            let l3_pa = PhysAddr::new(pte_address(l4_table[l4_idx]));
            let l3_table =
                &mut *((layout::DIRECTMAP_BASE + l3_pa.as_u64()) as *mut [u64; PAGE_TABLE_ENTRIES]);
            if !pte_is_present(l3_table[l3_idx]) {
                return Err(PagingError::PdptNotPresent);
            }
            if pte_is_huge(l3_table[l3_idx]) {
                let phys_addr = pte_address(l3_table[l3_idx]);
                l3_table[l3_idx] = descriptor::leaf(phys_addr, new_flags | PTE_HUGE_PAGE);
                flush_tlb_one_smp(va, mutation_asid(va, self.active_asid));
                return Ok(());
            }
            let l2_pa = PhysAddr::new(pte_address(l3_table[l3_idx]));
            let l2_table =
                &mut *((layout::DIRECTMAP_BASE + l2_pa.as_u64()) as *mut [u64; PAGE_TABLE_ENTRIES]);
            if !pte_is_present(l2_table[l2_idx]) {
                return Err(PagingError::PdNotPresent);
            }
            if pte_is_huge(l2_table[l2_idx]) {
                let phys_addr = pte_address(l2_table[l2_idx]);
                l2_table[l2_idx] = descriptor::leaf(phys_addr, new_flags | PTE_HUGE_PAGE);
                flush_tlb_one_smp(va, mutation_asid(va, self.active_asid));
                return Ok(());
            }
            let l1_pa = PhysAddr::new(pte_address(l2_table[l2_idx]));
            let l1_table =
                &mut *((layout::DIRECTMAP_BASE + l1_pa.as_u64()) as *mut [u64; PAGE_TABLE_ENTRIES]);
            if !pte_is_present(l1_table[l1_idx]) {
                return Err(PagingError::PtNotPresent);
            }
            let phys_addr = pte_address(l1_table[l1_idx]);
            l1_table[l1_idx] = descriptor::leaf(phys_addr, new_flags);
            flush_tlb_one_smp(va, mutation_asid(va, self.active_asid));
        }
        Ok(())
    }
}
