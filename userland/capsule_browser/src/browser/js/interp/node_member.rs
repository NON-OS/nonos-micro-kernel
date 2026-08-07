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

use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::dom::node::NodeKind;
use crate::browser::js::value::Value;

use super::attr_prop::{attr_prop, bool_prop};
use super::ctx::Ctx;
use super::node_text::node_text;

pub fn node_member(ctx: &mut Ctx, id: usize, name: &str) -> Value {
    if id >= ctx.dom.nodes.len() {
        return Value::Undef;
    }
    match name {
        "textContent" | "innerText" | "innerHTML" => Value::Str(Rc::new(node_text(ctx.dom, id))),
        "id" => Value::Str(Rc::new(ctx.dom.nodes[id].attr("id").unwrap_or("").to_string())),
        "className" => {
            Value::Str(Rc::new(ctx.dom.nodes[id].attr("class").unwrap_or("").to_string()))
        }
        "value" => Value::Str(Rc::new(ctx.dom.nodes[id].attr("value").unwrap_or("").to_string())),
        "tagName" | "nodeName" => Value::Str(Rc::new(ctx.dom.nodes[id].tag.to_ascii_uppercase())),
        "parentNode" | "parentElement" => Value::Node(ctx.dom.nodes[id].parent),
        // Navigation a reconciler walks on every update. `children` skips
        // text, and these deliberately do not: a framework that placed a
        // text node has to be able to find it again.
        "firstChild" => match ctx.dom.nodes[id].children.first() {
            Some(&c) => Value::Node(c),
            None => Value::Null,
        },
        "lastChild" => match ctx.dom.nodes[id].children.last() {
            Some(&c) => Value::Node(c),
            None => Value::Null,
        },
        "nextSibling" => sibling(ctx, id, 1),
        "previousSibling" => sibling(ctx, id, -1),
        // 1 for an element, 3 for text, as the specification numbers them.
        "nodeType" => Value::Num(match ctx.dom.nodes[id].kind {
            NodeKind::Element => 1.0,
            _ => 3.0,
        }),
        "classList" => Value::Bound("classList", id),
        "style" => Value::Bound("style", id),
        "children" => {
            let kids: Vec<Value> = ctx.dom.nodes[id]
                .children
                .iter()
                .filter(|&&c| ctx.dom.nodes.get(c).is_some_and(|n| n.kind == NodeKind::Element))
                .map(|&c| Value::Node(c))
                .collect();
            Value::Array(Rc::new(RefCell::new(kids)))
        }
        _ => reflected(ctx, id, name),
    }
}

/// A property that is really an attribute, read back through it.
fn reflected(ctx: &Ctx, id: usize, name: &str) -> Value {
    if let Some(attr) = bool_prop(name) {
        return Value::Bool(ctx.dom.nodes[id].attr(attr).is_some());
    }
    match attr_prop(name) {
        Some(attr) => Value::Str(Rc::new(ctx.dom.nodes[id].attr(attr).unwrap_or("").to_string())),
        None => Value::Undef,
    }
}

/// The sibling `step` places away, or null at either end.
///
/// Read from the parent's list rather than held on the node, so that a move
/// cannot leave a stale link behind: there is only ever one record of where
/// a node sits.
fn sibling(ctx: &Ctx, id: usize, step: isize) -> Value {
    let parent = ctx.dom.nodes[id].parent;
    let Some(node) = ctx.dom.nodes.get(parent) else {
        return Value::Null;
    };
    let Some(at) = node.children.iter().position(|&c| c == id) else {
        return Value::Null;
    };
    let Some(next) = at.checked_add_signed(step) else {
        return Value::Null;
    };
    match node.children.get(next) {
        Some(&c) => Value::Node(c),
        None => Value::Null,
    }
}
