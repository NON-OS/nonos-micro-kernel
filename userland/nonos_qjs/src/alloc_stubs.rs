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

//! C allocator ABI over the Rust global allocator. Each block carries a header
//! word holding its payload size so free, realloc and usable-size recover it
//! without a separate table.

use alloc::alloc::{alloc, dealloc, realloc as grow};
use core::alloc::Layout;
use core::ptr;

const HDR: usize = 16;

unsafe fn layout(total: usize) -> Layout {
    Layout::from_size_align_unchecked(total, 16)
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return ptr::null_mut();
    }
    let base = alloc(layout(size + HDR));
    if base.is_null() {
        return ptr::null_mut();
    }
    *(base as *mut usize) = size;
    base.add(HDR)
}

#[no_mangle]
pub unsafe extern "C" fn free(p: *mut u8) {
    if p.is_null() {
        return;
    }
    let base = p.sub(HDR);
    let size = *(base as *mut usize);
    dealloc(base, layout(size + HDR));
}

#[no_mangle]
pub unsafe extern "C" fn calloc(n: usize, sz: usize) -> *mut u8 {
    let total = n.saturating_mul(sz);
    let p = malloc(total);
    if !p.is_null() {
        ptr::write_bytes(p, 0, total);
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn realloc(p: *mut u8, size: usize) -> *mut u8 {
    if p.is_null() {
        return malloc(size);
    }
    if size == 0 {
        free(p);
        return ptr::null_mut();
    }
    let base = p.sub(HDR);
    let old = *(base as *mut usize);
    let nbase = grow(base, layout(old + HDR), size + HDR);
    if nbase.is_null() {
        return ptr::null_mut();
    }
    *(nbase as *mut usize) = size;
    nbase.add(HDR)
}

#[no_mangle]
pub unsafe extern "C" fn malloc_usable_size(p: *const u8) -> usize {
    if p.is_null() {
        return 0;
    }
    *(p.sub(HDR) as *const usize)
}

#[no_mangle]
pub unsafe extern "C" fn malloc_size(p: *const u8) -> usize {
    malloc_usable_size(p)
}
