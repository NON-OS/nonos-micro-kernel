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

use super::apply::apply;
use super::ctx::Ctx;
use super::eval_args::eval_args;
use super::eval_expr::eval_expr;

pub fn eval_call(ctx: &mut Ctx, env: &Env, callee: &Expr, arg_exprs: &[Expr]) -> Result<Value, ()> {
    if let Expr::Member(obj, method) = callee {
        let recv = eval_expr(ctx, env, obj)?;
        let argv = eval_args(ctx, env, arg_exprs)?;
        if let Value::Object(map) = &recv {
            let f = map.borrow().get(method).cloned();
            if let Some(f) = f {
                return apply(ctx, env, f, argv);
            }
        }
        return Ok(Value::Undef);
    }
    let f = eval_expr(ctx, env, callee)?;
    let argv = eval_args(ctx, env, arg_exprs)?;
    apply(ctx, env, f, argv)
}
