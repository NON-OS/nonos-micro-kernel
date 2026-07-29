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

use super::super::core::PagingManager;
use super::super::shootdown::flush_tlb_one_smp;
use super::super::tlb_scope::mutation_asid;
use crate::arch::paging::read_root as read_cr3;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::layout;
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::types::{PagePermissions, PageSize};

fn table_at(pa: PhysAddr) -> *mut [u64; PAGE_TABLE_ENTRIES] {
    (layout::DIRECTMAP_BASE + pa.as_u64()) as *mut [u64; PAGE_TABLE_ENTRIES]
}

impl PagingManager {
    pub fn unmap_page(
        &mut self,
        virtual_addr: VirtAddr,
    ) -> PagingResult<(PhysAddr, PagePermissions, PageSize)> {
        if !self.initialized {
            return Err(PagingError::NotInitialized);
        }
        let page_addr = page_align_down(virtual_addr.as_u64());
        let mapping = self.mappings.remove(&page_addr).ok_or(PagingError::PageNotMapped)?;
        let physical_addr = self.remove_mapping(virtual_addr)?;
        Ok((physical_addr, mapping.permissions, mapping.size))
    }

    pub(in crate::memory::paging::manager) fn remove_mapping(
        &self,
        va: VirtAddr,
    ) -> PagingResult<PhysAddr> {
        let va_val = va.as_u64();
        let (l4_idx, l3_idx, l2_idx, l1_idx) =
            (pml4_index(va_val), pdpt_index(va_val), pd_index(va_val), pt_index(va_val));
        // Live per-CPU CR3, not the racy cached field: on SMP the unmap must act
        // on the faulting CPU's own active address space (see install_mapping).
        let cr3 = PhysAddr::new(read_cr3() & !0xFFF);
        unsafe {
            let l4 = &*table_at(cr3);
            if !pte_is_present(l4[l4_idx]) {
                return Err(PagingError::Pml4NotPresent);
            }
            let l3 = &*table_at(PhysAddr::new(pte_address(l4[l4_idx])));
            if !pte_is_present(l3[l3_idx]) {
                return Err(PagingError::PdptNotPresent);
            }
            let l2 = &*table_at(PhysAddr::new(pte_address(l3[l3_idx])));
            if !pte_is_present(l2[l2_idx]) {
                return Err(PagingError::PdNotPresent);
            }
            let l1 = &mut *table_at(PhysAddr::new(pte_address(l2[l2_idx])));
            if !pte_is_present(l1[l1_idx]) {
                return Err(PagingError::PtNotPresent);
            }
            let pa = PhysAddr::new(pte_address(l1[l1_idx]));
            l1[l1_idx] = 0;
            // Scope the shootdown to the asid that owned the mapping;
            // fall back to a kernel-wide flush when the manager has
            // no active asid recorded (boot path before any process
            // is dispatched). Single-CPU runtime stays a local
            // `invlpg`; SMP runtime broadcasts to peer CPUs running
            // the same address space.
            let asid = mutation_asid(va, Some(crate::smp::percpu::active_asid()));
            flush_tlb_one_smp(va, asid);
            Ok(pa)
        }
    }
}
