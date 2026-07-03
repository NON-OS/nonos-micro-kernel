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

use crate::browser::js::ast::{Expr, Stmt};
use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::eval_expr::eval_expr;
use super::exec::exec;
use super::flow::Flow;

// for (var x of it): arrays iterate their items, strings their characters.
// The items snapshot up front so body mutations cannot livelock the loop.
pub fn eval_for_of(
    ctx: &mut Ctx,
    env: &Env,
    name: &str,
    iter: &Expr,
    body: &[Stmt],
) -> Result<Flow, ()> {
    let it = eval_expr(ctx, env, iter)?;
    let items: Vec<Value> = match it {
        Value::Array(a) => a.borrow().clone(),
        Value::Str(s) => s.chars().map(|c| Value::Str(Rc::new(c.to_string()))).collect(),
        _ => Vec::new(),
    };
    for item in items {
        if !ctx.tick() {
            return Err(());
        }
        let child = env.child();
        child.define(name, item);
        match exec(ctx, &child, body)? {
            Flow::Break => break,
            Flow::Return(v) => return Ok(Flow::Return(v)),
            _ => {}
        }
    }
    Ok(Flow::Normal)
}
