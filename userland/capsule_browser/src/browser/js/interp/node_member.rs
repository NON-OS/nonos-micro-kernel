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
        _ => Value::Undef,
    }
}
