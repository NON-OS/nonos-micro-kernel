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

//! Pre-seed every kernel-half PML4 entry with a backing PDPT.
//!
//! Walked once at boot from `create_kernel_address_space`, before
//! any user CR3 is cloned. Any PML4[256..511] entry that is not
//! already present gets a fresh, zeroed PDPT frame allocated and
//! installed with `present | writable`. The PDPTs are global state
//! shared by every address space cloned afterwards; sub-table
//! allocations (PD, PT, leaf) flow into them and propagate to all
//! address spaces without further bookkeeping.

use crate::arch::paging::descriptor;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc;
use crate::memory::paging::constants::{pte_is_present, PAGE_TABLE_ENTRIES};
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::unified::phys_to_virt_checked;

const PML4_KERNEL_HALF_START: usize = PAGE_TABLE_ENTRIES / 2;

pub(super) fn seed_kernel_half_pdpts(kernel_cr3: PhysAddr) -> PagingResult<()> {
    let pml4_va = phys_to_virt_checked(kernel_cr3).ok_or(PagingError::NoActivePageTable)?;
    let pml4 = unsafe { &mut *(pml4_va.as_u64() as *mut [u64; PAGE_TABLE_ENTRIES]) };
    for i in PML4_KERNEL_HALF_START..PAGE_TABLE_ENTRIES {
        if pte_is_present(pml4[i]) {
            continue;
        }
        let pdpt_frame = frame_alloc::allocate_frame().ok_or(PagingError::FrameAllocationFailed)?;
        let pdpt_va = phys_to_virt_checked(pdpt_frame).ok_or(PagingError::NoActivePageTable)?;
        let pdpt = unsafe { &mut *(pdpt_va.as_u64() as *mut [u64; PAGE_TABLE_ENTRIES]) };
        for entry in pdpt.iter_mut() {
            *entry = 0;
        }
        pml4[i] = descriptor::table(pdpt_frame.as_u64(), false);
    }
    Ok(())
}
