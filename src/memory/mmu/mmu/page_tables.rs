// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::super::constants::PAGE_TABLE_ENTRIES;
use super::super::error::{MmuError, MmuResult};
use super::core::MMU;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc;
use core::arch::asm;

impl MMU {
    pub(super) fn setup_initial_page_tables(&self) -> MmuResult<()> {
        let pml4 = self.allocate_page_table_frame()?;
        let pml4_va = self.frame_to_virt(pml4);
        unsafe {
            core::ptr::write_bytes(pml4_va.as_mut_ptr::<u64>(), 0, PAGE_TABLE_ENTRIES);
        }
        self.load_page_table(pml4)?;
        Ok(())
    }

    pub(super) fn load_page_table(&self, pml4_frame: PhysAddr) -> MmuResult<()> {
        // Installing a freshly built table, so no address-space id to carry:
        // zero keeps the classic flush-on-write behaviour.
        crate::arch::paging::write_root(pml4_frame.as_u64(), 0);
        *self.current_cr3.lock() = pml4_frame.as_u64();
        self.invalidate_tlb_all();
        Ok(())
    }

    pub(super) fn allocate_page_table_frame(&self) -> MmuResult<PhysAddr> {
        frame_alloc::allocate_frame().ok_or(MmuError::FrameAllocationFailed)
    }
}
