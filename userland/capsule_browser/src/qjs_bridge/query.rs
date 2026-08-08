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

//! Asking a selector about one node, and reading a subtree back as markup.

use core::ffi::c_void;

use crate::browser::css;
use crate::qjs_dom::{cdup, cstr, dom};

/// Whether this node matches a selector.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_matches(host: *mut c_void, node: i32, sel: *const u8) -> i32 {
    if node < 0 {
        return 0;
    }
    css::matches(dom(host), node as usize, &cstr(sel)) as i32
}

/// The nearest node at or above this one that matches, or -1.
///
/// This is how a page turns a click on whatever was under the pointer into
/// the row or link the handler is about, so it runs on every delegated
/// event. A page that delegates and cannot call it handles nothing.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_closest(host: *mut c_void, node: i32, sel: *const u8) -> i32 {
    if node < 0 {
        return -1;
    }
    css::closest(dom(host), node as usize, &cstr(sel)).map(|i| i as i32).unwrap_or(-1)
}

/// The markup inside a node. Only the setter existed, so reading gave
/// undefined and code that copies one part of a page into another wrote the
/// string "undefined" back out.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_inner_html(host: *mut c_void, node: i32) -> *mut u8 {
    if node < 0 {
        return core::ptr::null_mut();
    }
    cdup(&dom(host).inner_html(node as usize))
}

/// The node itself and the markup inside it.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_get_outer_html(host: *mut c_void, node: i32) -> *mut u8 {
    if node < 0 {
        return core::ptr::null_mut();
    }
    cdup(&dom(host).outer_html(node as usize))
}

/// How many attributes a node carries, so the names can be walked.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_attr_count(host: *mut c_void, node: i32) -> i32 {
    if node < 0 {
        return 0;
    }
    dom(host).nodes.get(node as usize).map(|n| n.attrs.len() as i32).unwrap_or(0)
}

/// The name of the attribute at a position, for getAttributeNames and the
/// dataset walk.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_attr_name_at(host: *mut c_void, node: i32, i: i32) -> *mut u8 {
    if node < 0 || i < 0 {
        return core::ptr::null_mut();
    }
    match dom(host).nodes.get(node as usize).and_then(|n| n.attrs.get(i as usize)) {
        Some((k, _)) => cdup(k),
        None => core::ptr::null_mut(),
    }
}
