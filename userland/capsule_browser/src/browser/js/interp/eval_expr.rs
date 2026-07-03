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

use crate::browser::js::ast::Expr;
use crate::browser::js::env::Env;
use crate::browser::js::value::{FuncData, Value};

use super::ctx::Ctx;
use super::eval_array::eval_array;
use super::eval_assign::eval_assign;
use super::eval_binary::eval_binary;
use super::eval_call::eval_call;
use super::eval_index::eval_index;
use super::eval_logical::eval_logical;
use super::eval_member::eval_member;
use super::eval_object::eval_object;
use super::eval_unary::eval_unary;
use super::to_bool::to_bool;

pub fn eval_expr(ctx: &mut Ctx, env: &Env, e: &Expr) -> Result<Value, ()> {
    if !ctx.tick() {
        return Err(());
    }
    ctx.depth += 1;
    if ctx.depth > 900 {
        return Err(());
    }
    let v = match e {
        Expr::Num(n) => Value::Num(*n),
        Expr::Str(s) => Value::Str(Rc::new(s.clone())),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Null => Value::Null,
        Expr::Undef => Value::Undef,
        Expr::Ident(name) => env.get(name).unwrap_or(Value::Undef),
        Expr::Array(items) => eval_array(ctx, env, items)?,
        Expr::Object(props) => eval_object(ctx, env, props)?,
        Expr::Unary(op, a) => eval_unary(ctx, env, op, a)?,
        Expr::Binary(op, l, r) => eval_binary(ctx, env, op, l, r)?,
        Expr::Logical(op, l, r) => eval_logical(ctx, env, op, l, r)?,
        Expr::Assign(op, t, v) => eval_assign(ctx, env, op, t, v)?,
        Expr::Member(o, name) => eval_member(ctx, env, o, name)?,
        Expr::Index(o, i) => eval_index(ctx, env, o, i)?,
        Expr::Call(c, a) => eval_call(ctx, env, c, a)?,
        Expr::Ternary(c, a, b) => {
            if to_bool(&eval_expr(ctx, env, c)?) {
                eval_expr(ctx, env, a)?
            } else {
                eval_expr(ctx, env, b)?
            }
        }
        Expr::Func(params, body) => Value::Func(Rc::new(FuncData {
            params: params.clone(),
            body: body.clone(),
            env: env.clone(),
        })),
    };
    ctx.depth -= 1;
    Ok(v)
}
