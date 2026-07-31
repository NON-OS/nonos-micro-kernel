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

//! An allocator that clears a block before returning it to the free list.
//!
//! Capsules hold the secrets a user would mind losing: the keyring's stored
//! keys, the wallet's note secrets and signing material, TLS traffic keys. A
//! `Drop` on the owning type covers the original, but not the copies that a
//! `clone()`, a protocol encode buffer or a `Vec` growth leave behind. Those
//! reach `dealloc` as ordinary bytes, and a plain free list overwrites only
//! the first few bytes of the block with its own links. The rest stays
//! readable to the next allocation that lands there, and stays in DRAM.
//!
//! Wiping on free rather than on allocation is deliberate: it bounds how long
//! a secret is readable to the moment it is dropped, instead of to whenever
//! that memory happens to be reused, which may be never.
//!
//! This costs a write per freed byte. That is the price of the guarantee and
//! it is paid on every capsule free, not only the ones holding secrets, since
//! the allocator cannot know which is which.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

use linked_list_allocator::LockedHeap;

pub struct ZeroOnFree {
    inner: LockedHeap,
}

impl ZeroOnFree {
    pub const fn empty() -> Self {
        Self { inner: LockedHeap::empty() }
    }

    /// Hand the backing region to the inner allocator.
    ///
    /// # Safety
    /// `base` must point at `bytes` of memory that is writable, unaliased, and
    /// lives as long as the allocator, which is the whole run.
    pub unsafe fn init(&self, base: *mut u8, bytes: usize) {
        // SAFETY: the caller guarantees the region; this only forwards it.
        unsafe { self.inner.lock().init(base, bytes) }
    }
}

// SAFETY: every method forwards to `LockedHeap`, which is itself a sound
// `GlobalAlloc`. `dealloc` writes only within the block the caller is giving
// up, using the layout it was allocated with, and does so before the block
// re-enters the free list, so no live allocation is touched.
unsafe impl GlobalAlloc for ZeroOnFree {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwarded unchanged.
        unsafe { self.inner.alloc(layout) }
    }

    unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
        if !block.is_null() {
            // Volatile, because the block is dead as far as the compiler is
            // concerned: a plain `write_bytes` here is a store to memory
            // nothing reads again, which it is free to drop entirely.
            let len = layout.size();
            for i in 0..len {
                // SAFETY: `i < len` and the caller states the block is `len`
                // bytes it owns and is releasing, so every byte is in bounds
                // and unaliased.
                unsafe { ptr::write_volatile(block.add(i), 0) };
            }
        }
        // SAFETY: forwarded unchanged, after the block was cleared in place.
        unsafe { self.inner.dealloc(block, layout) }
    }
}
