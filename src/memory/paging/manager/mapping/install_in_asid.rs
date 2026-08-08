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
use super::super::shootdown::{flush_tlb_one_smp, ASID_KERNEL};
use super::super::tlb_scope::is_kernel_half;
use crate::arch::paging::descriptor;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::{frame_alloc, layout};

fn alloc_table(entry: &mut u64) -> PagingResult<()> {
    let new = frame_alloc::allocate_frame().ok_or(PagingError::FrameAllocationFailed)?;
    // The intermediate levels impose no restriction; the leaf decides.
    *entry = descriptor::table(new.as_u64(), true);
    unsafe {
        core::ptr::write_bytes((layout::DIRECTMAP_BASE + new.as_u64()) as *mut u8, 0, PAGE_SIZE_4K);
    }
    Ok(())
}

fn table_at(pa: PhysAddr) -> *mut [u64; PAGE_TABLE_ENTRIES] {
    (layout::DIRECTMAP_BASE + pa.as_u64()) as *mut [u64; PAGE_TABLE_ENTRIES]
}

impl PagingManager {
    pub(in crate::memory::paging::manager) fn install_mapping_in_asid(
        &self,
        asid: u32,
        va: VirtAddr,
        pa: PhysAddr,
        flags: u64,
    ) -> PagingResult<()> {
        let address_space =
            self.address_spaces.get(&asid).ok_or(PagingError::AddressSpaceNotFound)?;
        let cr3 = address_space.cr3_value;
        let va_val = va.as_u64();
        let (l4_idx, l3_idx, l2_idx, l1_idx) =
            (pml4_index(va_val), pdpt_index(va_val), pd_index(va_val), pt_index(va_val));

        // SAFETY: eK@nonos.systems — cr3 is one of ours, page tables
        // go through the directmap.
        unsafe {
            let l4 = &mut *table_at(cr3);
            if !pte_is_present(l4[l4_idx]) {
                alloc_table(&mut l4[l4_idx])?;
            }
            let l3 = &mut *table_at(PhysAddr::new(pte_address(l4[l4_idx])));
            if !pte_is_present(l3[l3_idx]) {
                alloc_table(&mut l3[l3_idx])?;
            }
            let l2 = &mut *table_at(PhysAddr::new(pte_address(l3[l3_idx])));
            if !pte_is_present(l2[l2_idx]) {
                alloc_table(&mut l2[l2_idx])?;
            }
            let l1 = &mut *table_at(PhysAddr::new(pte_address(l2[l2_idx])));
            l1[l1_idx] = descriptor::leaf(pa.as_u64(), flags);
        }

        if is_kernel_half(va) {
            flush_tlb_one_smp(va, ASID_KERNEL);
        } else if self.active_asid == Some(asid) {
            flush_tlb_one_smp(va, asid);
        }
        Ok(())
    }

    pub(in crate::memory::paging::manager) fn translate_in_asid(
        &self,
        asid: u32,
        va: VirtAddr,
    ) -> Option<PhysAddr> {
        let cr3 = self.address_spaces.get(&asid)?.cr3_value;
        let v = va.as_u64();
        // SAFETY: eK@nonos.systems — cr3 is one of ours; every table
        // is reached through the directmap and only read here.
        unsafe {
            let e4 = (*table_at(cr3))[pml4_index(v)];
            if !pte_is_present(e4) {
                return None;
            }
            let e3 = (*table_at(PhysAddr::new(pte_address(e4))))[pdpt_index(v)];
            if !pte_is_present(e3) {
                return None;
            }
            if pte_is_huge(e3) {
                return Some(PhysAddr::new(pte_address(e3) + (v & 0x3fff_ffff)));
            }
            let e2 = (*table_at(PhysAddr::new(pte_address(e3))))[pd_index(v)];
            if !pte_is_present(e2) {
                return None;
            }
            if pte_is_huge(e2) {
                return Some(PhysAddr::new(pte_address(e2) + (v & 0x1f_ffff)));
            }
            let e1 = (*table_at(PhysAddr::new(pte_address(e2))))[pt_index(v)];
            if !pte_is_present(e1) {
                return None;
            }
            Some(PhysAddr::new(pte_address(e1) + (v & 0xfff)))
        }
    }
}
