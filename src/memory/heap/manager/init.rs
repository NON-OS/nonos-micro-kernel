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

use super::super::error::{HeapError, HeapResult};
use super::globals::{HEAP_STATS, KERNEL_HEAP, USING_BOOTSTRAP};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager;
use crate::memory::paging::types::PagePermissions;
use crate::memory::{frame_alloc, layout};
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

pub fn init() -> HeapResult<()> {
    if KERNEL_HEAP.is_initialized() {
        return Ok(());
    }
    let heap_size = layout::KHEAP_SIZE as usize;
    let heap_pages = (heap_size + layout::PAGE_SIZE - 1) / layout::PAGE_SIZE;
    let heap_frames = allocate_heap_frames(heap_pages)?;
    let heap_start = map_heap_memory(&heap_frames)?;
    unsafe {
        KERNEL_HEAP.init(heap_start, heap_size);
    }
    HEAP_STATS.set_total_size(heap_size);
    USING_BOOTSTRAP.store(false, Ordering::Release);
    Ok(())
}

fn allocate_heap_frames(page_count: usize) -> HeapResult<Vec<PhysAddr>> {
    let mut frames = Vec::with_capacity(page_count);
    for _ in 0..page_count {
        match frame_alloc::allocate_frame() {
            Some(addr) => frames.push(addr),
            None => return Err(HeapError::FrameAllocationFailed),
        }
    }
    Ok(frames)
}

fn map_heap_memory(frames: &[PhysAddr]) -> HeapResult<*mut u8> {
    let heap_start = VirtAddr::new(layout::KHEAP_BASE);
    let perms = PagePermissions::READ | PagePermissions::WRITE;
    for (i, &frame_addr) in frames.iter().enumerate() {
        let virt_addr = VirtAddr::new(heap_start.as_u64() + (i * layout::PAGE_SIZE) as u64);
        manager::map_page(virt_addr, frame_addr, perms).map_err(|_| HeapError::MappingFailed)?;
    }
    Ok(heap_start.as_mut_ptr())
}
