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

use crate::browser::js::ast::Stmt;
use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::class_build::build_class;
use super::ctx::Ctx;
use super::eval_expr::eval_expr;
use super::eval_for::eval_for;
use super::eval_for_of::eval_for_of;
use super::eval_if::eval_if;
use super::eval_try::eval_try;
use super::eval_while::eval_while;
use super::exec::exec;
use super::flow::Flow;

pub fn eval_stmt(ctx: &mut Ctx, env: &Env, s: &Stmt) -> Result<Flow, ()> {
    if !ctx.tick() {
        return Err(());
    }
    match s {
        Stmt::Expr(e) => {
            eval_expr(ctx, env, e)?;
            Ok(Flow::Normal)
        }
        Stmt::Var(decls) => {
            for (name, init) in decls {
                let v = match init {
                    Some(e) => eval_expr(ctx, env, e)?,
                    None => Value::Undef,
                };
                env.define(name, v);
            }
            Ok(Flow::Normal)
        }
        Stmt::If(c, t, e) => eval_if(ctx, env, c, t, e),
        Stmt::While(c, b) => eval_while(ctx, env, c, b),
        Stmt::For(i, c, u, b) => eval_for(ctx, env, i, c, u, b),
        Stmt::ForOf(name, iter, b) => eval_for_of(ctx, env, name, iter, b),
        Stmt::Func(..) => Ok(Flow::Normal),
        Stmt::Return(e) => {
            let v = match e {
                Some(x) => eval_expr(ctx, env, x)?,
                None => Value::Undef,
            };
            Ok(Flow::Return(v))
        }
        Stmt::Break => Ok(Flow::Break),
        Stmt::Continue => Ok(Flow::Continue),
        Stmt::Block(b) => {
            let child = env.child();
            exec(ctx, &child, b)
        }
        Stmt::Class(name, sup, methods) => {
            let cls = build_class(ctx, env, sup, methods)?;
            env.define(name, cls);
            Ok(Flow::Normal)
        }
        Stmt::Throw(e) => {
            let v = eval_expr(ctx, env, e)?;
            ctx.exception = Some(v);
            Err(())
        }
        Stmt::Try(body, catch, finally) => eval_try(ctx, env, body, catch, finally),
    }
}
