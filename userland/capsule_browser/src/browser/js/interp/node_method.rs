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

use crate::browser::css;
use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::in_subtree::in_subtree;
use super::to_str::to_str;

const QUERY_CAP: usize = 256;

// Element methods. Unknown methods answer undefined so scripts keep going.
pub(super) fn node_method(ctx: &mut Ctx, id: usize, method: &str, argv: &[Value]) -> Value {
    if id >= ctx.dom.nodes.len() {
        return Value::Undef;
    }
    match method {
        "getAttribute" => {
            let name = argv.first().map(to_str).unwrap_or_default();
            match ctx.dom.nodes[id].attr(&name) {
                Some(v) => Value::Str(Rc::new(v.to_string())),
                None => Value::Null,
            }
        }
        "setAttribute" => {
            let name = argv.first().map(to_str).unwrap_or_default();
            let value = argv.get(1).map(to_str).unwrap_or_default();
            if !name.is_empty() {
                ctx.dom.set_attr(id, &name, value);
                ctx.dirty = true;
            }
            Value::Undef
        }
        "removeAttribute" => {
            let name = argv.first().map(to_str).unwrap_or_default();
            if !name.is_empty() {
                ctx.dom.remove_attr(id, &name);
                ctx.dirty = true;
            }
            Value::Undef
        }
        "hasAttribute" => {
            let name = argv.first().map(to_str).unwrap_or_default();
            Value::Bool(ctx.dom.nodes[id].attr(&name).is_some())
        }
        "appendChild" => {
            if let Some(Value::Node(child)) = argv.first() {
                if ctx.dom.place(id, *child, usize::MAX) {
                    ctx.dirty = true;
                    return Value::Node(*child);
                }
            }
            Value::Undef
        }
        // A framework reorders by inserting ahead of a sibling. Appending can
        // only build a list once; this is what keeps it correct after that.
        "insertBefore" => {
            if let Some(Value::Node(child)) = argv.first() {
                let before = match argv.get(1) {
                    Some(Value::Node(r)) => *r,
                    // A null reference means append, which is what the
                    // caller asks for when it is adding at the end.
                    _ => usize::MAX,
                };
                if ctx.dom.place(id, *child, before) {
                    ctx.dirty = true;
                    return Value::Node(*child);
                }
            }
            Value::Undef
        }
        "replaceChild" => {
            if let (Some(Value::Node(fresh)), Some(Value::Node(old))) = (argv.first(), argv.get(1))
            {
                let (fresh, old) = (*fresh, *old);
                if ctx.dom.nodes.get(old).is_some_and(|n| n.parent == id)
                    && ctx.dom.insert_before(id, fresh, old)
                {
                    ctx.dom.detach(old);
                    ctx.dirty = true;
                    return Value::Node(old);
                }
            }
            Value::Undef
        }
        "removeChild" => {
            if let Some(Value::Node(child)) = argv.first() {
                let child = *child;
                if ctx.dom.nodes.get(child).is_some_and(|n| n.parent == id) {
                    ctx.dom.detach(child);
                    ctx.dirty = true;
                    return Value::Node(child);
                }
            }
            Value::Undef
        }
        // A page writes a row's shape once in markup and every row is a copy
        // of it. Without this a script has to build each one tag by tag.
        "cloneNode" => {
            let deep = matches!(argv.first(), Some(Value::Bool(true)));
            match ctx.dom.clone_node(id, deep) {
                Some(copy) => Value::Node(copy),
                None => Value::Undef,
            }
        }
        "contains" => match argv.first() {
            Some(Value::Node(other)) => Value::Bool(in_subtree(ctx.dom, id, *other)),
            _ => Value::Bool(false),
        },
        "remove" => {
            ctx.dom.detach(id);
            ctx.dirty = true;
            Value::Undef
        }
        "addEventListener" => {
            let event = argv.first().map(to_str).unwrap_or_default();
            if !event.is_empty() && event.len() <= 32 && ctx.listeners.len() < 512 {
                if let Some(cb @ Value::Func(_)) = argv.get(1) {
                    ctx.listeners.push((id, event, cb.clone()));
                }
            }
            Value::Undef
        }
        "querySelector" => {
            let sel = argv.first().map(to_str).unwrap_or_default();
            for hit in css::select(ctx.dom, &sel, QUERY_CAP) {
                if in_subtree(ctx.dom, id, hit) {
                    return Value::Node(hit);
                }
            }
            Value::Null
        }
        "querySelectorAll" => {
            let sel = argv.first().map(to_str).unwrap_or_default();
            let hits: Vec<Value> = css::select(ctx.dom, &sel, QUERY_CAP)
                .into_iter()
                .filter(|&hit| in_subtree(ctx.dom, id, hit))
                .map(Value::Node)
                .collect();
            Value::Array(Rc::new(RefCell::new(hits)))
        }
        _ => Value::Undef,
    }
}
