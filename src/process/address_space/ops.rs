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

use super::tlb::invlpg;
use super::types::{
    pte_flags, AddressSpace, PageTable, PageTableEntry, ProtectionFlags, KERNEL_SPACE_START,
    USER_SPACE_END,
};
use crate::memory::addr::{PhysAddr, VirtAddr};

#[inline(always)]
pub fn pml4_index(virt: VirtAddr) -> usize {
    ((virt.as_u64() >> 39) & 0x1FF) as usize
}

#[inline(always)]
pub fn pdpt_index(virt: VirtAddr) -> usize {
    ((virt.as_u64() >> 30) & 0x1FF) as usize
}

#[inline(always)]
pub fn pd_index(virt: VirtAddr) -> usize {
    ((virt.as_u64() >> 21) & 0x1FF) as usize
}

#[inline(always)]
pub fn pt_index(virt: VirtAddr) -> usize {
    ((virt.as_u64() >> 12) & 0x1FF) as usize
}

pub fn current_pml4() -> *mut PageTable {
    let phys = crate::arch::paging::read_root() & pte_flags::ADDR_MASK;
    (phys + KERNEL_SPACE_START) as *mut PageTable
}

pub fn get_or_create_table(table: &mut PageTable, index: usize) -> Result<PhysAddr, &'static str> {
    let entry = table.entry_mut(index);

    if entry.is_present() {
        Ok(entry.phys_addr())
    } else {
        let frame = crate::memory::phys::alloc(crate::memory::phys::AllocFlags::empty())
            .ok_or("Failed to allocate page table")?;

        let ptr = (frame.0 + KERNEL_SPACE_START) as *mut PageTable;
        // SAFETY: The physical frame was just allocated from the frame allocator,
        // so it is valid memory. We add KERNEL_SPACE_START to convert the physical
        // address to its kernel-mapped virtual address. The PageTable struct is
        // repr(C) and zero-initializing it produces a valid, empty page table.
        unsafe {
            (*ptr).zero();
        }

        *entry = PageTableEntry::new(
            PhysAddr::new(frame.0),
            pte_flags::PRESENT | pte_flags::WRITABLE | pte_flags::USER_ACCESSIBLE,
        );

        Ok(PhysAddr::new(frame.0))
    }
}

impl AddressSpace {
    pub fn map_page(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        prot: ProtectionFlags,
    ) -> Result<(), &'static str> {
        if virt.as_u64() >= USER_SPACE_END && !self.is_kernel {
            return Err("Cannot map kernel address in user space");
        }

        let flags = prot.to_pte_flags();

        let pml4_ptr = (self.pml4_phys.as_u64() + KERNEL_SPACE_START) as *mut PageTable;

        let pml4_idx = pml4_index(virt);
        let pdpt_idx = pdpt_index(virt);
        let pd_idx = pd_index(virt);
        let pt_idx = pt_index(virt);

        // SAFETY: This block walks and modifies the page table hierarchy.
        // - pml4_phys is valid because it comes from self.pml4_phys which was
        //   allocated during AddressSpace::new()
        // - Adding KERNEL_SPACE_START converts physical to kernel virtual address
        // - All intermediate page tables are either already present or allocated
        //   via get_or_create_table() which validates the allocation
        // - The final entry write maps a physical page to a virtual address
        unsafe {
            let pdpt_phys = get_or_create_table(&mut *pml4_ptr, pml4_idx)?;
            let pdpt_ptr = (pdpt_phys.as_u64() + KERNEL_SPACE_START) as *mut PageTable;

            let pd_phys = get_or_create_table(&mut *pdpt_ptr, pdpt_idx)?;
            let pd_ptr = (pd_phys.as_u64() + KERNEL_SPACE_START) as *mut PageTable;

            let pt_phys = get_or_create_table(&mut *pd_ptr, pd_idx)?;
            let pt_ptr = (pt_phys.as_u64() + KERNEL_SPACE_START) as *mut PageTable;

            let entry = (*pt_ptr).entry_mut(pt_idx);
            *entry = PageTableEntry::new(phys, flags);
        }

        Ok(())
    }

    pub fn unmap_page(&mut self, virt: VirtAddr) -> Result<PhysAddr, &'static str> {
        let pml4_ptr = (self.pml4_phys.as_u64() + KERNEL_SPACE_START) as *mut PageTable;

        let pml4_idx = pml4_index(virt);
        let pdpt_idx = pdpt_index(virt);
        let pd_idx = pd_index(virt);
        let pt_idx = pt_index(virt);

        // SAFETY: This block walks the page table hierarchy to remove a mapping.
        // - pml4_phys is valid because it comes from self.pml4_phys
        // - Adding KERNEL_SPACE_START converts physical to kernel virtual address
        // - Each level is validated for presence before proceeding
        // - The final entry clear removes the mapping and invlpg flushes the TLB
        unsafe {
            let pml4_entry = (*pml4_ptr).entry(pml4_idx);
            if !pml4_entry.is_present() {
                return Err("Page not mapped (PML4)");
            }

            let pdpt_ptr = (pml4_entry.phys_addr().as_u64() + KERNEL_SPACE_START) as *mut PageTable;
            let pdpt_entry = (*pdpt_ptr).entry(pdpt_idx);
            if !pdpt_entry.is_present() {
                return Err("Page not mapped (PDPT)");
            }

            let pd_ptr = (pdpt_entry.phys_addr().as_u64() + KERNEL_SPACE_START) as *mut PageTable;
            let pd_entry = (*pd_ptr).entry(pd_idx);
            if !pd_entry.is_present() {
                return Err("Page not mapped (PD)");
            }

            let pt_ptr = (pd_entry.phys_addr().as_u64() + KERNEL_SPACE_START) as *mut PageTable;
            let pt_entry = (*pt_ptr).entry_mut(pt_idx);
            if !pt_entry.is_present() {
                return Err("Page not mapped (PT)");
            }

            let phys = pt_entry.phys_addr();
            pt_entry.clear();

            invlpg(virt);

            Ok(phys)
        }
    }
}
