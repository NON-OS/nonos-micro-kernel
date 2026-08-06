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

use crate::browser::dom::node::NodeKind;
use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::super::to_str::to_str;

// document.createElement: a detached element that renders once appended.
pub fn create_element(ctx: &mut Ctx, argv: &[Value]) -> Value {
    let tag = argv.first().map(to_str).unwrap_or_default().to_ascii_lowercase();
    if tag.is_empty() || tag.len() > 32 || !tag.bytes().all(|b| b.is_ascii_alphanumeric()) {
        return Value::Null;
    }
    match ctx.dom.create(NodeKind::Element, tag) {
        Some(id) => Value::Node(id),
        None => Value::Null,
    }
}

/// document.createTextNode: a detached text node that renders once appended.
///
/// A framework builds text through this rather than by assigning to a
/// parent, because it has to hold on to the node to update it in place
/// later. Without it every string a component renders has nowhere to go.
pub fn create_text_node(ctx: &mut Ctx, argv: &[Value]) -> Value {
    let text = argv.first().map(to_str).unwrap_or_default();
    match ctx.dom.create(NodeKind::Text, alloc::string::String::new()) {
        Some(id) => {
            ctx.dom.nodes[id].text = text;
            Value::Node(id)
        }
        None => Value::Null,
    }
}

/// document.createDocumentFragment: a holder that vanishes when it is placed.
///
/// A fragment exists so a script can build several nodes and put them in
/// with one call. It is never part of the document itself: appending it
/// appends its children and leaves the holder behind, which is what the
/// placement path does when it is handed one.
pub fn create_fragment(ctx: &mut Ctx) -> Value {
    match ctx.dom.create(NodeKind::Document, alloc::string::String::new()) {
        Some(id) => Value::Node(id),
        None => Value::Null,
    }
}
