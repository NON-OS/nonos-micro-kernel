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

//! Swapping one node for another, and asking whether an attribute is there.

use core::ffi::c_void;

use crate::qjs_dom::{cstr, dom};

/// Put `fresh` where `old` sits and take `old` out.
///
/// A framework that replaces a node rather than editing it needs the new one
/// to land in the same position. Appending and then removing would put it at
/// the end, so a replaced row jumps to the bottom of the list.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_replace_child(
    host: *mut c_void,
    parent: i32,
    fresh: i32,
    old: i32,
) -> i32 {
    if parent < 0 || fresh < 0 || old < 0 {
        return -1;
    }
    let (p, f, o) = (parent as usize, fresh as usize, old as usize);
    let d = dom(host);
    if d.nodes.get(o).map(|n| n.parent) != Some(p) {
        return -1;
    }
    if !d.place(p, f, o) {
        return -1;
    }
    d.detach(o);
    old
}

/// Whether the attribute is present, without reading its value.
///
/// A present attribute with an empty value is how markup writes the ones
/// that are simply on, so asking for the value cannot answer this.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_has_attr(host: *mut c_void, node: i32, k: *const u8) -> i32 {
    if node < 0 {
        return 0;
    }
    let key = cstr(k);
    match dom(host).nodes.get(node as usize) {
        Some(n) => n.attrs.iter().any(|(a, _)| *a == key) as i32,
        None => 0,
    }
}

/// Whether `other` is `node` or sits somewhere beneath it.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_contains(host: *mut c_void, node: i32, other: i32) -> i32 {
    if node < 0 || other < 0 {
        return 0;
    }
    let (root, mut at) = (node as usize, other as usize);
    let d = dom(host);
    for _ in 0..super::MAX_ANCESTRY {
        if at == root {
            return 1;
        }
        match d.nodes.get(at).map(|n| n.parent) {
            Some(p) if p != at => at = p,
            _ => return 0,
        }
    }
    0
}
