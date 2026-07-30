// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use super::limits::DISPLAY_FRAMEBUFFER_PAGES;
use crate::memory::phys::{alloc_contiguous, AllocFlags, PAGE_SIZE_U64};
use spin::Mutex;

const POOL_CAP_PAGES: usize = DISPLAY_FRAMEBUFFER_PAGES as usize;
const WORD_BITS: usize = 64;
const WORDS: usize = POOL_CAP_PAGES / WORD_BITS;

struct DisplayPool {
    base: u64,
    pages: usize,
    used: [u64; WORDS],
}

impl DisplayPool {
    const fn new() -> Self {
        Self { base: 0, pages: 0, used: [0; WORDS] }
    }
}

static DISPLAY_POOL: Mutex<DisplayPool> = Mutex::new(DisplayPool::new());

pub(crate) fn init_display_pool() {
    let mut pool = DISPLAY_POOL.lock();
    if pool.pages != 0 {
        return;
    }
    for pages in [POOL_CAP_PAGES, 4096, 2048, 1024] {
        if let Some(base) = alloc_contiguous(pages, AllocFlags::DMA | AllocFlags::HIGH) {
            pool.base = base;
            pool.pages = pages;
            crate::sys::serial::print(b"[DMA] display pool base=");
            crate::sys::serial::print_hex(base);
            crate::sys::serial::print(b" pages=");
            crate::sys::serial::print_dec(pages as u64);
            crate::sys::serial::println(b"");
            return;
        }
    }
    crate::sys::serial::println(b"[DMA] display pool unavailable");
}

pub(super) fn alloc(pages: usize) -> Option<u64> {
    let mut pool = DISPLAY_POOL.lock();
    if pages == 0 || pool.pages == 0 || pages > pool.pages {
        return None;
    }
    let mut run = 0usize;
    let mut start = 0usize;
    for idx in 0..pool.pages {
        if pool.used[idx / WORD_BITS] & (1u64 << (idx % WORD_BITS)) == 0 {
            if run == 0 {
                start = idx;
            }
            run += 1;
            if run == pages {
                for page in start..start + pages {
                    pool.used[page / WORD_BITS] |= 1u64 << (page % WORD_BITS);
                }
                return Some(pool.base + (start as u64 * PAGE_SIZE_U64));
            }
        } else {
            run = 0;
        }
    }
    None
}

pub(super) fn free(addr: u64, pages: usize) -> bool {
    let mut pool = DISPLAY_POOL.lock();
    if pages == 0 || pool.pages == 0 || addr < pool.base {
        return false;
    }
    let offset = match addr.checked_sub(pool.base) {
        Some(v) if v % PAGE_SIZE_U64 == 0 => v as usize / PAGE_SIZE_U64 as usize,
        _ => return false,
    };
    if offset.checked_add(pages).is_none_or(|end| end > pool.pages) {
        return false;
    }
    for page in offset..offset + pages {
        pool.used[page / WORD_BITS] &= !(1u64 << (page % WORD_BITS));
    }
    true
}

// A reserved pool of below-4GB memory for 32-bit DMA devices. Regular allocations
// consume low memory bottom-up, so by the time a wifi driver initialises there is
// often nothing left below 4GB; reserving this pool early guarantees a home for
// buffers named by a 32-bit descriptor. Sized for device rings and small staging
// buffers, not bulk data (storage is 64-bit capable and never comes here).
const LOW32_POOL_PAGES: usize = 2048; // 8MB
const LOW32_WORDS: usize = LOW32_POOL_PAGES / WORD_BITS;
/// The largest single request served from the low pool; larger DMA is 64-bit
/// capable and goes to the general allocator instead of draining the reserve.
const LOW32_MAX_PAGES: usize = 256; // 1MB

struct Low32Pool {
    base: u64,
    pages: usize,
    used: [u64; LOW32_WORDS],
}

impl Low32Pool {
    const fn new() -> Self {
        Self { base: 0, pages: 0, used: [0; LOW32_WORDS] }
    }
}

static LOW32_POOL: Mutex<Low32Pool> = Mutex::new(Low32Pool::new());

/// The capacity, in pages, the low pool can track (bounded by its static bitmap).
pub(crate) const fn low32_capacity_pages() -> usize {
    LOW32_POOL_PAGES
}

/// Record the below-4GB physical range `[base, base + pages*PAGE)` as the DMA
/// pool. The caller carves this from a low usable memory region the main
/// allocator does not manage, so the two never hand out the same frame. `base`
/// zero (no low region found) leaves the pool unavailable.
pub(crate) fn init_low32_pool(base: u64, pages: usize) {
    let mut pool = LOW32_POOL.lock();
    if pool.pages != 0 {
        return;
    }
    let pages = pages.min(LOW32_POOL_PAGES);
    if base == 0 || pages == 0 {
        crate::sys::serial::println(b"[DMA] low32 pool unavailable (no low region)");
        return;
    }
    pool.base = base;
    pool.pages = pages;
    crate::sys::serial::print(b"[DMA] low32 pool base=");
    crate::sys::serial::print_hex(base);
    crate::sys::serial::print(b" pages=");
    crate::sys::serial::print_dec(pages as u64);
    crate::sys::serial::println(b"");
}

/// Allocate `pages` from the low pool, or `None` if it is too large for the pool
/// policy, the pool is uninitialised, or it is full.
pub(super) fn low32_alloc(pages: usize) -> Option<u64> {
    if pages == 0 || pages > LOW32_MAX_PAGES {
        return None;
    }
    let mut pool = LOW32_POOL.lock();
    if pool.pages == 0 || pages > pool.pages {
        return None;
    }
    let mut run = 0usize;
    let mut start = 0usize;
    for idx in 0..pool.pages {
        if pool.used[idx / WORD_BITS] & (1u64 << (idx % WORD_BITS)) == 0 {
            if run == 0 {
                start = idx;
            }
            run += 1;
            if run == pages {
                for page in start..start + pages {
                    pool.used[page / WORD_BITS] |= 1u64 << (page % WORD_BITS);
                }
                return Some(pool.base + (start as u64 * PAGE_SIZE_U64));
            }
        } else {
            run = 0;
        }
    }
    None
}

/// Whether `addr` was handed out by the low pool, so a free is routed here.
pub(super) fn low32_owns(addr: u64) -> bool {
    let pool = LOW32_POOL.lock();
    pool.pages != 0 && addr >= pool.base && addr < pool.base + (pool.pages as u64 * PAGE_SIZE_U64)
}

pub(super) fn low32_free(addr: u64, pages: usize) -> bool {
    let mut pool = LOW32_POOL.lock();
    if pages == 0 || pool.pages == 0 || addr < pool.base {
        return false;
    }
    let offset = match addr.checked_sub(pool.base) {
        Some(v) if v % PAGE_SIZE_U64 == 0 => v as usize / PAGE_SIZE_U64 as usize,
        _ => return false,
    };
    if offset.checked_add(pages).is_none_or(|end| end > pool.pages) {
        return false;
    }
    for page in offset..offset + pages {
        pool.used[page / WORD_BITS] &= !(1u64 << (page % WORD_BITS));
    }
    true
}
