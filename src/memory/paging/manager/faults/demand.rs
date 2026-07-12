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

use crate::memory::addr::VirtAddr;

use super::super::core::PagingManager;
use crate::memory::paging::constants::PAGE_SIZE_4K;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::stats::PagingStatistics;
use crate::memory::paging::types::{PagePermissions, PageSize};
use crate::memory::{frame_alloc, layout};

impl PagingManager {
    pub(super) fn handle_demand_fault(
        &mut self,
        virtual_addr: VirtAddr,
        stats: &PagingStatistics,
    ) -> PagingResult<()> {
        // Only user-space addresses may be demand-backed. A not-present fault
        // in the kernel half is never a legitimate lazy mapping; backing it
        // silently would hand a capsule kernel-range memory. Surface it as an
        // unhandled fault so the fault path kills the offender (user) or traps
        // the real kernel bug, instead of papering over it.
        if !layout::in_user_space(virtual_addr.as_u64()) {
            return Err(PagingError::UnhandledPageFault);
        }

        // Never demand-back the null page. A fault in the lowest page is a null
        // or near-null dereference; backing it would silently satisfy the bug
        // instead of trapping it. Leave the page unmapped as a guard so the
        // fault path kills the offending capsule.
        if virtual_addr.as_u64() < PAGE_SIZE_4K as u64 {
            return Err(PagingError::UnhandledPageFault);
        }

        // Charge the page against the faulting process's demand budget. A
        // runaway capsule is refused here and killed by the fault path instead
        // of exhausting physical memory.
        let pid = crate::process::current_pid().unwrap_or(0);
        if !super::demand_cap::charge(pid) {
            return Err(PagingError::UnhandledPageFault);
        }

        let new_frame = frame_alloc::allocate_frame().ok_or(PagingError::FrameAllocationFailed)?;

        unsafe {
            let va = layout::DIRECTMAP_BASE + new_frame.as_u64();
            core::ptr::write_bytes(va as *mut u8, 0, PAGE_SIZE_4K);
        }

        let permissions = PagePermissions::READ | PagePermissions::WRITE | PagePermissions::USER;
        self.map_page(virtual_addr, new_frame, permissions, PageSize::Size4KiB, stats)?;

        Ok(())
    }
}
