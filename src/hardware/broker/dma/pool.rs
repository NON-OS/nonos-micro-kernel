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
