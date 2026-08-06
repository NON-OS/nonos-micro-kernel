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

use crate::browser::js::ast::Expr;
use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::bound_member::bound_member;
use super::ctx::Ctx;
use super::eval_expr::eval_expr;
use super::node_member::node_member;

pub fn eval_member(ctx: &mut Ctx, env: &Env, obj: &Expr, name: &str) -> Result<Value, ()> {
    let recv = eval_expr(ctx, env, obj)?;
    Ok(match recv {
        Value::Object(map) => {
            let b = map.borrow();
            // The document names nodes in the live tree, which did not exist
            // when this object was built.
            if b.contains_key("__document__") {
                if let Some(v) = super::document_member::document_member(ctx, name) {
                    return Ok(v);
                }
            }
            if name == "size" {
                if let Some(Value::Array(s)) = b.get("__map__").or_else(|| b.get("__set__")) {
                    return Ok(Value::Num(s.borrow().len() as f64));
                }
            }
            b.get(name).cloned().unwrap_or(Value::Undef)
        }
        Value::Array(a) => {
            if name == "length" {
                Value::Num(a.borrow().len() as f64)
            } else {
                Value::Undef
            }
        }
        Value::Str(s) => {
            if name == "length" {
                Value::Num(s.chars().count() as f64)
            } else {
                Value::Undef
            }
        }
        Value::Node(id) => node_member(ctx, id, name),
        Value::Bound(kind, id) => bound_member(ctx, kind, id, name),
        _ => Value::Undef,
    })
}
