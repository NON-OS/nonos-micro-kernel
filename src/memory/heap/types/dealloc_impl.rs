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

use super::super::constants::MIN_ALIGNMENT;
use super::allocator::SecureHeapAllocator;
use super::header::AllocationHeader;
use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;
use core::sync::atomic::Ordering;

// Counterpart of `alloc_impl`: the pointer was produced by an allocation
// aligned to `layout.align().max(MIN_ALIGNMENT)` ≥ 8.
#[allow(clippy::cast_ptr_alignment)]
pub(super) unsafe fn dealloc_impl(allocator: &SecureHeapAllocator, ptr: *mut u8, layout: Layout) {
    unsafe {
        if ptr.is_null() || !allocator.is_initialized() {
            return;
        }

        let header_size = mem::size_of::<AllocationHeader>();
        let raw_ptr = ptr.sub(header_size);
        let header_ptr = raw_ptr as *const AllocationHeader;

        let header = ptr::read_volatile(header_ptr);
        if !header.is_valid() || header.size != layout.size() {
            #[cfg(feature = "nonos-heap-debug")]
            heap_corrupt_log(b"hdr", ptr as u64, header.size as u64, layout.size() as u64);
            return;
        }

        let canary_ptr = ptr.add(header.canary_offset) as *const u64;
        let canary = ptr::read_volatile(canary_ptr);
        if canary != allocator.canary_value {
            #[cfg(feature = "nonos-heap-debug")]
            heap_corrupt_log(b"canary", ptr as u64, canary, allocator.canary_value);
            return;
        }

        if super::super::manager::HEAP_ZERO_ON_FREE.load(Ordering::Relaxed) {
            ptr::write_bytes(ptr, 0, layout.size());
        }

        let total_size = header_size + layout.size() + mem::size_of::<u64>();
        let align = layout.align().max(MIN_ALIGNMENT);
        if let Ok(adjusted_layout) = Layout::from_size_align(total_size, align) {
            super::super::manager::HEAP_STATS.record_deallocation(layout.size());
            allocator.inner.dealloc(raw_ptr, adjusted_layout);
        }
    }
}

#[cfg(feature = "nonos-heap-debug")]
fn heap_corrupt_log(kind: &[u8], ptr: u64, got: u64, want: u64) {
    use core::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    if N.fetch_add(1, Ordering::Relaxed) >= 40 {
        return;
    }
    crate::sys::serial::print(b"[HEAP-CORRUPT ");
    crate::sys::serial::print(kind);
    crate::sys::serial::print(b"] ptr=");
    crate::arch::x86_64::diag::print_hex_u64(ptr);
    crate::sys::serial::print(b" got=");
    crate::arch::x86_64::diag::print_hex_u64(got);
    crate::sys::serial::print(b" want=");
    crate::arch::x86_64::diag::print_hex_u64(want);
    crate::sys::serial::println(b"");
}
