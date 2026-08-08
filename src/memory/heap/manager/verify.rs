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

#[cfg(feature = "heap-track")]
use super::super::types::AllocationHeader;
#[cfg(feature = "heap-track")]
use super::globals::get_timestamp;
use super::globals::KERNEL_HEAP;
#[cfg(feature = "heap-track")]
use crate::memory::layout;
#[cfg(feature = "heap-track")]
use core::{mem, ptr};

pub fn verify_heap_integrity() -> bool {
    if !KERNEL_HEAP.is_initialized() {
        return false;
    }
    let heap_size = KERNEL_HEAP.get_heap_size();
    if heap_size == 0 {
        return false;
    }
    verify_tracked_allocations()
}

// Without `heap-track` no live-pointer set exists to walk, so this reduces to a
// no-op. The per-free header and redzone checks in `dealloc_impl` are always
// compiled and remain the live corruption detectors.
#[cfg(not(feature = "heap-track"))]
fn verify_tracked_allocations() -> bool {
    true
}

#[cfg(feature = "heap-track")]
fn verify_tracked_allocations() -> bool {
    let _sampled = KERNEL_HEAP.tracking_overflowed.load(core::sync::atomic::Ordering::Relaxed);
    crate::arch::run_without_interrupts(|| {
        let allocated_ptrs = KERNEL_HEAP.allocated_ptrs.lock();
        for &ptr_addr in allocated_ptrs.iter() {
            if ptr_addr < layout::KHEAP_BASE as usize
                || ptr_addr >= (layout::KHEAP_BASE + layout::KHEAP_SIZE) as usize
            {
                return false;
            }
            unsafe {
                let header_size = mem::size_of::<AllocationHeader>();
                let header_ptr = (ptr_addr - header_size) as *const AllocationHeader;
                let header = ptr::read_volatile(header_ptr);
                if !header.is_valid() {
                    return false;
                }
                let canary_ptr = (ptr_addr + header.canary_offset) as *const u64;
                let canary = ptr::read_volatile(canary_ptr);
                if canary != KERNEL_HEAP.canary_value {
                    return false;
                }
                let current_time = get_timestamp();
                if current_time < header.allocated_at {
                    return false;
                }
            }
        }
        true
    })
}
