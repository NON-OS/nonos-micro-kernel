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

use super::ctx::Ctx;

/// Members of `document` that name a node rather than hold a value.
///
/// The rest of the object is built once, before any document exists, which
/// is why these could not be among them: the node each one names is not
/// known until a page has been parsed, and changes when another is loaded.
/// So they are answered when they are read.
///
/// `document.body.appendChild` is how most scripts reach the page at all,
/// and it was answering undefined.
pub fn document_member(ctx: &Ctx, name: &str) -> Option<Value> {
    let tag = match name {
        "body" => "body",
        "head" => "head",
        "documentElement" => "html",
        _ => return None,
    };
    if let Some(id) = find(ctx, tag) {
        return Some(Value::Node(id));
    }
    // Markup that never wrote the tag still has the content it would have
    // held, and a script asking for somewhere to put a node should be given
    // one rather than turned away. The root is where that content already
    // lives.
    match name {
        "body" | "documentElement" => Some(Value::Node(0)),
        _ => Some(Value::Null),
    }
}

fn find(ctx: &Ctx, tag: &str) -> Option<usize> {
    ctx.dom.nodes.iter().position(|n| n.kind == NodeKind::Element && n.tag == tag)
}
