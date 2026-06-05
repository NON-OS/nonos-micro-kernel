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

//! Kernel-half clone for a freshly allocated PML4 frame.
//!
//! The user half (entries 0..255) is zeroed so a new address space
//! carries no inherited user mappings. The kernel half (entries
//! 256..511) is copied verbatim from KERNEL_ASID's PML4. Sourcing
//! the half from `self.active_page_table` would be unsafe once the
//! scheduler runs because that field tracks whatever user CR3 is
//! currently loaded.

use super::super::core::PagingManager;
use crate::memory::addr::PhysAddr;
use crate::memory::paging::constants::{KERNEL_ASID, PAGE_TABLE_ENTRIES};
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::unified::phys_to_virt_checked;

const PML4_KERNEL_HALF_START: usize = PAGE_TABLE_ENTRIES / 2;

impl PagingManager {
    pub(super) fn clone_kernel_half_into(&self, page_table_pa: PhysAddr) -> PagingResult<()> {
        let page_table_va =
            phys_to_virt_checked(page_table_pa).ok_or(PagingError::NoActivePageTable)?;
        let page_table =
            unsafe { &mut *(page_table_va.as_u64() as *mut [u64; PAGE_TABLE_ENTRIES]) };
        for entry in page_table.iter_mut() {
            *entry = 0;
        }
        let kernel_space = self
            .address_spaces
            .get(&KERNEL_ASID)
            .ok_or(PagingError::NoActivePageTable)?;
        let kernel_table_va =
            phys_to_virt_checked(kernel_space.cr3_value).ok_or(PagingError::NoActivePageTable)?;
        let kernel_table =
            unsafe { &*(kernel_table_va.as_u64() as *const [u64; PAGE_TABLE_ENTRIES]) };
        let mut populated = 0usize;
        for i in PML4_KERNEL_HALF_START..PAGE_TABLE_ENTRIES {
            page_table[i] = kernel_table[i];
            if kernel_table[i] != 0 {
                populated += 1;
            }
        }
        if populated == 0 {
            return Err(PagingError::NoActivePageTable);
        }
        Ok(())
    }
}
