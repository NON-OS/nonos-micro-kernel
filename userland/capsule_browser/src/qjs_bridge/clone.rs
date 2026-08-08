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

//! Copying a node, and the holder a script fills before placing several.

use alloc::string::String;
use core::ffi::c_void;

use crate::browser::dom::node::NodeKind;
use crate::qjs_dom::dom;

/// Copy a node and hand back the copy, unattached.
///
/// A page writes a row's shape once in markup and every row is a copy of it
/// filled in. Without this a script has to build each row tag by tag, which
/// is not what the code on the page does, so the list comes out empty.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_clone_node(host: *mut c_void, node: i32, deep: i32) -> i32 {
    if node < 0 {
        return -1;
    }
    dom(host).clone_node(node as usize, deep != 0).map(|i| i as i32).unwrap_or(-1)
}

/// A holder a script fills so several nodes go in with one call.
///
/// It is never part of the document: placing it puts its children in and
/// leaves it empty, which is what `place` does with one.
#[no_mangle]
pub unsafe extern "C" fn njs_dom_create_fragment(host: *mut c_void) -> i32 {
    dom(host).create(NodeKind::Document, String::new()).map(|i| i as i32).unwrap_or(-1)
}
