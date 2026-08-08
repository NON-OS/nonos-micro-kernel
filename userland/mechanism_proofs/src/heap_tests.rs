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

//! A freed capsule allocation is unreadable afterwards.
//!
//! Runs the real ZeroOnFree over a region this test owns so the block can be
//! read back after dealloc. Without the wipe, 48 of 64 secret bytes survived.
//! Volatile both ways: the compiler may assume nothing reads a freed block, so
//! a plain read folds against the writes and passes regardless.

use core::alloc::{GlobalAlloc, Layout};

use crate::heap::zero_on_free::ZeroOnFree;

const SECRET: u8 = 0xAB;

/// Allocate, fill with `SECRET`, free, and report how many secret bytes are
/// still there. The region leaks deliberately: the allocator holds it for the
/// rest of the process.
fn surviving_bytes_after_free(size: usize) -> usize {
    const REGION: usize = 1 << 16;
    let backing: *mut u8 = Box::leak(vec![0u8; REGION].into_boxed_slice()).as_mut_ptr();
    let allocator = ZeroOnFree::empty();
    // SAFETY: the region is freshly allocated, unaliased, and leaked, so it
    // outlives the allocator that is about to own it.
    unsafe { allocator.init(backing, REGION) };

    let layout = Layout::from_size_align(size, 8).expect("valid layout");
    // SAFETY: the allocator was just initialised over a region far larger than
    // this request.
    let block = unsafe { allocator.alloc(layout) };
    assert!(!block.is_null(), "allocation failed");

    for i in 0..size {
        // SAFETY: `i < size` and the block is `size` bytes this test owns.
        unsafe { core::ptr::write_volatile(block.add(i), SECRET) };
    }
    // SAFETY: the block came from this allocator with this layout and is not
    // used as a live allocation again; the reads below are of freed memory the
    // test still owns through `backing`.
    unsafe { allocator.dealloc(block, layout) };

    (0..size)
        .filter(|&i| {
            // SAFETY: within the leaked region, which is still mapped.
            unsafe { core::ptr::read_volatile(block.add(i)) == SECRET }
        })
        .count()
}

#[test]
fn a_freed_capsule_allocation_keeps_no_secret() {
    for size in [16usize, 64, 256, 4096] {
        assert_eq!(
            surviving_bytes_after_free(size),
            0,
            "a {size} byte block still held its contents after being freed"
        );
    }
}

#[test]
fn the_wipe_covers_sizes_that_are_not_multiples_of_a_word() {
    // The loop is per byte, so an odd tail must be covered too. A word-at-a
    // time wipe that rounded down would leave the last bytes readable.
    for size in [1usize, 7, 9, 63, 65, 4095] {
        assert_eq!(surviving_bytes_after_free(size), 0, "a {size} byte block kept its tail");
    }
}
