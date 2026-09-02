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

//! Where this MMU handle gets its root from, and where its intermediate
//! tables come from.
//!
//! The root is adopted, never built. The bootloader hands off a table that
//! already maps kernel text, the directmap and the heap, and the kernel runs
//! out of it from the first instruction. Allocating a fresh root here and
//! loading it would unmap the code doing the loading, so the only correct
//! answer is to read what is live and record it.

use super::super::error::{MmuError, MmuResult};
use super::core::MMU;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc;

impl MMU {
    /// Record the page table the CPU is already translating through.
    pub(super) fn adopt_active_root(&self) -> MmuResult<()> {
        let root = crate::arch::paging::read_root();
        if root == 0 {
            return Err(MmuError::NoPageTableLoaded);
        }
        *self.current_cr3.lock() = root;
        Ok(())
    }

    pub(super) fn allocate_page_table_frame(&self) -> MmuResult<PhysAddr> {
        frame_alloc::allocate_frame().ok_or(MmuError::FrameAllocationFailed)
    }
}
