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
        "className" => Value::Str(Rc::new(ctx.dom.nodes[id].attr("class").unwrap_or("").to_string())),
        "tagName" | "nodeName" => Value::Str(Rc::new(ctx.dom.nodes[id].tag.to_ascii_uppercase())),
        "parentNode" => Value::Node(ctx.dom.nodes[id].parent),
        _ => Value::Undef,
    }
}
