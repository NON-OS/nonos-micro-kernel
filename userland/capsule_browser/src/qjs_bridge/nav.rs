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

//! Walking the tree in the directions the bridge did not cover.

use core::ffi::c_void;

use crate::qjs_dom::dom;

/// The last child, or -1 when there are none.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_last_child(host: *mut c_void, node: i32) -> i32 {
    if node < 0 {
        return -1;
    }
    match dom(host).nodes.get(node as usize) {
        Some(n) => n.children.last().map(|&c| c as i32).unwrap_or(-1),
        None => -1,
    }
}

/// The sibling before this one, or -1 at the front of the list.
///
/// Read from the parent's list rather than held on the node, so a move
/// cannot leave a stale link: there is one record of where a node sits.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_prev_sibling(host: *mut c_void, node: i32) -> i32 {
    if node <= 0 {
        return -1;
    }
    let d = dom(host);
    let Some(parent) = d.nodes.get(node as usize).map(|n| n.parent) else {
        return -1;
    };
    let Some(p) = d.nodes.get(parent) else {
        return -1;
    };
    let Some(at) = p.children.iter().position(|&c| c == node as usize) else {
        return -1;
    };
    match at.checked_sub(1) {
        Some(prev) => p.children.get(prev).map(|&c| c as i32).unwrap_or(-1),
        None => -1,
    }
}
