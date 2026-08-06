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

//! Where the page came from, and what a relative address means from there.

use core::ffi::c_void;
use core::ptr;

use crate::browser::url;
use crate::qjs_dom::{cdup, cstr, dom};

/// The address the document was loaded from.
///
/// `location` reported `http://localhost/` no matter what had been fetched.
/// A page that reads its own path to decide what to show, which is every
/// page with a router in it, was told something that was never true.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_base_url(host: *mut c_void) -> *mut u8 {
    cdup(&dom(host).base)
}

/// A relative address made absolute against the page.
///
/// `href` and `src` came back exactly as markup spelled them, so a script
/// reading a link got "/about" where it expected the whole address, and
/// anything comparing it against a real one disagreed.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_resolve(host: *mut c_void, rel: *const u8) -> *mut u8 {
    let d = dom(host);
    let relative = cstr(rel);
    if relative.is_empty() {
        return ptr::null_mut();
    }
    // Without a base there is nothing to resolve against, and guessing one
    // would turn a relative address into a confident wrong one.
    match url::parse(&d.base) {
        Some(base) => cdup(&url::join(&base, &relative)),
        None => cdup(&relative),
    }
}

/// One number from a node's laid-out box, as `getBoundingClientRect` and the
/// offset properties read it.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_box(host: *mut c_void, node: i32, which: i32) -> i32 {
    if node < 0 || which < 0 {
        return 0;
    }
    dom(host).box_of(node as usize, which as usize)
}
