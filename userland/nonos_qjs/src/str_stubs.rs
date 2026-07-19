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

//! The C string routines QuickJS needs that compiler_builtins does not provide.
//! (memcpy/memset/memmove/memcmp come from compiler_builtins-mem.)

use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut n = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

#[no_mangle]
pub unsafe extern "C" fn strchr(s: *const u8, c: i32) -> *const u8 {
    let t = c as u8;
    let mut p = s;
    loop {
        let b = *p;
        if b == t {
            return p;
        }
        if b == 0 {
            return ptr::null();
        }
        p = p.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn strrchr(s: *const u8, c: i32) -> *const u8 {
    let t = c as u8;
    let mut p = s;
    let mut last = ptr::null();
    loop {
        let b = *p;
        if b == t {
            last = p;
        }
        if b == 0 {
            return last;
        }
        p = p.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let (x, y) = (*a.add(i), *b.add(i));
        if x != y {
            return x as i32 - y as i32;
        }
        if x == 0 {
            return 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn strstr(hay: *const u8, needle: *const u8) -> *const u8 {
    if *needle == 0 {
        return hay;
    }
    let nlen = strlen(needle);
    let mut p = hay;
    while *p != 0 {
        let mut k = 0;
        while k < nlen && *p.add(k) == *needle.add(k) {
            k += 1;
        }
        if k == nlen {
            return p;
        }
        p = p.add(1);
    }
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn memchr(s: *const u8, c: i32, n: usize) -> *const u8 {
    let t = c as u8;
    let mut i = 0;
    while i < n {
        if *s.add(i) == t {
            return s.add(i);
        }
        i += 1;
    }
    ptr::null()
}
