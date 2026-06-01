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

//! Per-process VA allocator for `MkMmap`. A bump cursor between
//! [`USER_MMAP_BASE`] and [`USER_MMAP_END`] gives fresh ranges; a
//! sorted free list reclaims released ranges and coalesces
//! neighbours so a long-running capsule that mmaps and munmaps
//! does not fragment its address space.
//!
//! Caller-supplied (`addr != 0`) mappings bypass the allocator;
//! the allocator only owns ranges it handed out from `addr == 0`
//! requests.

extern crate alloc;

use alloc::vec::Vec;

const PAGE_SIZE: u64 = 4096;
// Must sit above the PIE load window so an anonymous mmap never lands on
// a capsule's own image: ELF bases span [DEFAULT_PIE_BASE, + EXEC range]
// = [0x40_0000, 0x4040_0000) and images extend further, so the old
// 0x4000_0000 base let the heap map over a capsule loaded near the top.
const USER_MMAP_BASE: u64 = 0x0000_8000_0000;
const USER_MMAP_END: u64 = 0x0000_7000_0000_0000;

#[derive(Clone, Copy)]
struct Range {
    base: u64,
    pages: u64,
}

pub struct MmapVa {
    bump: u64,
    free: Vec<Range>,
    allocated: Vec<Range>,
}

impl MmapVa {
    pub const fn new() -> Self {
        Self { bump: USER_MMAP_BASE, free: Vec::new(), allocated: Vec::new() }
    }

    pub fn reserve(&mut self, pages: u64) -> Option<u64> {
        if pages == 0 {
            return None;
        }
        let bytes = pages.checked_mul(PAGE_SIZE)?;
        let base = self.take_from_free(pages, bytes).or_else(|| self.take_from_bump(bytes))?;
        self.record_allocated(base, pages);
        Some(base)
    }

    pub fn release(&mut self, base: u64, pages: u64) -> bool {
        if pages == 0 {
            return false;
        }
        if !self.drop_allocated(base, pages) {
            return false;
        }
        let bytes = pages * PAGE_SIZE;
        if base + bytes == self.bump {
            self.bump = base;
            self.absorb_tail();
        } else {
            self.insert_free(base, pages);
        }
        true
    }

    fn take_from_free(&mut self, pages: u64, bytes: u64) -> Option<u64> {
        for i in 0..self.free.len() {
            if self.free[i].pages >= pages {
                let base = self.free[i].base;
                if self.free[i].pages == pages {
                    self.free.remove(i);
                } else {
                    self.free[i].base += bytes;
                    self.free[i].pages -= pages;
                }
                return Some(base);
            }
        }
        None
    }

    fn take_from_bump(&mut self, bytes: u64) -> Option<u64> {
        let base = self.bump;
        let end = base.checked_add(bytes)?;
        if end > USER_MMAP_END {
            return None;
        }
        self.bump = end;
        Some(base)
    }

    fn record_allocated(&mut self, base: u64, pages: u64) {
        let pos = self.allocated.binary_search_by_key(&base, |r| r.base).unwrap_or_else(|p| p);
        self.allocated.insert(pos, Range { base, pages });
    }

    fn drop_allocated(&mut self, base: u64, pages: u64) -> bool {
        match self.allocated.binary_search_by_key(&base, |r| r.base) {
            Ok(pos) if self.allocated[pos].pages == pages => {
                self.allocated.remove(pos);
                true
            }
            _ => false,
        }
    }

    fn absorb_tail(&mut self) {
        while let Some(last) = self.free.last().copied() {
            if last.base + last.pages * PAGE_SIZE == self.bump {
                self.bump = last.base;
                self.free.pop();
            } else {
                break;
            }
        }
    }

    fn insert_free(&mut self, base: u64, pages: u64) {
        let pos = self.free.binary_search_by_key(&base, |r| r.base).unwrap_or_else(|p| p);
        self.free.insert(pos, Range { base, pages });
        if pos + 1 < self.free.len() {
            let cur = self.free[pos];
            let next = self.free[pos + 1];
            if cur.base + cur.pages * PAGE_SIZE == next.base {
                self.free[pos].pages += next.pages;
                self.free.remove(pos + 1);
            }
        }
        if pos > 0 {
            let prev = self.free[pos - 1];
            let cur = self.free[pos];
            if prev.base + prev.pages * PAGE_SIZE == cur.base {
                self.free[pos - 1].pages += cur.pages;
                self.free.remove(pos);
            }
        }
    }
}
