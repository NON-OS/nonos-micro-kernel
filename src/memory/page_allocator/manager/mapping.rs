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

use super::super::error::{PageAllocError, PageAllocResult};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::buddy_alloc;
use crate::memory::paging::manager;

pub(super) fn allocate_virtual_pages(page_count: usize) -> PageAllocResult<VirtAddr> {
    buddy_alloc::allocate_pages(page_count).map_err(|e| match e {
        buddy_alloc::BuddyAllocError::InvalidPageCount => PageAllocError::InvalidSize,
        buddy_alloc::BuddyAllocError::AllocationTooLarge => PageAllocError::TooManyPages,
        buddy_alloc::BuddyAllocError::OutOfVirtualMemory => PageAllocError::OutOfVirtualSpace,
        buddy_alloc::BuddyAllocError::FrameAllocationFailed => {
            PageAllocError::FrameAllocationFailed
        }
        buddy_alloc::BuddyAllocError::MappingFailed => PageAllocError::MappingFailed,
        buddy_alloc::BuddyAllocError::NotInitialized => PageAllocError::NotInitialized,
        _ => PageAllocError::TranslationFailed,
    })
}

pub(super) fn free_virtual_pages(va: VirtAddr, page_count: usize) -> PageAllocResult<()> {
    buddy_alloc::free_pages(va, page_count).map_err(|e| match e {
        buddy_alloc::BuddyAllocError::InvalidPageCount => PageAllocError::InvalidSize,
        buddy_alloc::BuddyAllocError::InvalidAddress | buddy_alloc::BuddyAllocError::DoubleFree => {
            PageAllocError::PageNotFound
        }
        buddy_alloc::BuddyAllocError::UnmapFailed => PageAllocError::UnmapFailed,
        buddy_alloc::BuddyAllocError::NotInitialized => PageAllocError::NotInitialized,
        _ => PageAllocError::TranslationFailed,
    })
}

pub(super) fn get_physical_address(va: VirtAddr) -> PageAllocResult<PhysAddr> {
    manager::translate_address(va).ok_or(PageAllocError::TranslationFailed)
}
