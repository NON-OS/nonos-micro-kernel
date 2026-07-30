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
use crate::memory::paging::constants::{page_align_down, PAGE_SIZE_4K};
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::stats::PagingStatistics;
use crate::memory::paging::types::{PagePermissions, PageSize};
use crate::memory::{frame_alloc, layout};

impl PagingManager {
    pub(super) fn handle_cow_fault(
        &mut self,
        virtual_addr: VirtAddr,
        stats: &PagingStatistics,
    ) -> PagingResult<()> {
        // A write fault on a *present* page is legitimate only when that page
        // was mapped copy-on-write. Any other present+write fault is a
        // protection violation: a write to a read-only page (a RELRO'd GOT,
        // .rodata, or code) or a supervisor write to a kernel page. Neither may
        // be silently promoted to writable, so fail closed here and let the
        // fault handler kill the offending capsule (or halt on a kernel fault).
        if !layout::in_user_space(virtual_addr.as_u64()) {
            return Err(PagingError::UnhandledPageFault);
        }
        let page_addr = page_align_down(virtual_addr.as_u64());
        let original =
            self.mappings.get(&page_addr).ok_or(PagingError::UnhandledPageFault)?.permissions;
        if !original.contains(PagePermissions::COW) {
            return Err(PagingError::UnhandledPageFault);
        }

        let new_frame = frame_alloc::allocate_frame().ok_or(PagingError::FrameAllocationFailed)?;

        if let Ok(original_pa) = self.translate_address(virtual_addr) {
            unsafe {
                let src_va = layout::DIRECTMAP_BASE + original_pa.as_u64();
                let dst_va = layout::DIRECTMAP_BASE + new_frame.as_u64();
                core::ptr::copy_nonoverlapping(
                    src_va as *const u8,
                    dst_va as *mut u8,
                    PAGE_SIZE_4K,
                );
            }
        }

        // Resolve the copy-on-write: drop the COW marker and grant the deferred
        // write, preserving the original permissions (never fabricating USER).
        let permissions = original.remove(PagePermissions::COW).insert(PagePermissions::WRITE);
        self.map_page(virtual_addr, new_frame, permissions, PageSize::Size4KiB, stats)?;

        Ok(())
    }
}
