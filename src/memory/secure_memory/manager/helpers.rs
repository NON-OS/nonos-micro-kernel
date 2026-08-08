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

use crate::memory::addr::{PhysAddr, VirtAddr};
use core::sync::atomic::Ordering;

use crate::memory::buddy_alloc as mem_alloc;
use crate::memory::layout;
use crate::memory::paging::manager;

use super::super::constants::SECURE_SCRUB_PATTERN;
use super::super::error::{SecureMemoryError, SecureMemoryResult};
use super::super::types::SecurityLevel;

pub(super) fn allocate_virtual_memory(size: usize) -> SecureMemoryResult<VirtAddr> {
    let page_count = (size + layout::PAGE_SIZE - 1) / layout::PAGE_SIZE;
    mem_alloc::allocate_pages(page_count).map_err(|_| SecureMemoryError::AllocationFailed)
}

pub(super) fn free_virtual_memory(va: VirtAddr, size: usize) -> SecureMemoryResult<()> {
    let page_count = (size + layout::PAGE_SIZE - 1) / layout::PAGE_SIZE;
    mem_alloc::free_pages(va, page_count).map_err(|_| SecureMemoryError::AllocationFailed)
}

pub(super) fn get_physical_address(va: VirtAddr) -> SecureMemoryResult<PhysAddr> {
    manager::translate_address(va).ok_or(SecureMemoryError::TranslationFailed)
}

pub(super) fn get_timestamp() -> u64 {
    crate::arch::read_time_counter()
}

pub(super) fn secure_zero_memory(
    va: VirtAddr,
    size: usize,
    security_level: SecurityLevel,
) -> SecureMemoryResult<()> {
    let passes = security_level.scrub_passes();
    if passes == 0 {
        volatile_memset(va, 0, size);
    } else {
        for pass in 0..passes {
            let pattern = if pass % 2 == 0 { SECURE_SCRUB_PATTERN } else { !SECURE_SCRUB_PATTERN };
            volatile_memset(va, pattern, size);
            core::sync::atomic::compiler_fence(Ordering::SeqCst);
        }
        volatile_memset(va, 0, size);
        core::sync::atomic::compiler_fence(Ordering::SeqCst);
    }
    Ok(())
}

#[inline(never)]
fn volatile_memset(va: VirtAddr, value: u8, size: usize) {
    let ptr = va.as_mut_ptr::<u8>();
    for i in 0..size {
        unsafe {
            core::ptr::write_volatile(ptr.add(i), value);
        }
    }
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
}

pub(super) fn zero_on_alloc(va: VirtAddr, size: usize) {
    volatile_memset(va, 0, size);
}
