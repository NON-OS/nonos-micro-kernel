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

use alloc::boxed::Box;

use crate::browser::js::ast::{Expr, Stmt};
use crate::browser::js::env::Env;

use super::ctx::Ctx;
use super::eval_expr::eval_expr;
use super::eval_stmt::eval_stmt;
use super::exec::exec;
use super::flow::Flow;
use super::to_bool::to_bool;

pub fn eval_for(
    ctx: &mut Ctx,
    env: &Env,
    init: &Option<Box<Stmt>>,
    cond: &Option<Expr>,
    update: &Option<Expr>,
    body: &[Stmt],
) -> Result<Flow, ()> {
    let scope = env.child();
    if let Some(i) = init {
        eval_stmt(ctx, &scope, i)?;
    }
    loop {
        if !ctx.tick() {
            return Err(());
        }
        if let Some(c) = cond {
            let cv = eval_expr(ctx, &scope, c)?;
            if !to_bool(&cv) {
                break;
            }
        }
        let child = scope.child();
        match exec(ctx, &child, body)? {
            Flow::Break => break,
            Flow::Return(v) => return Ok(Flow::Return(v)),
            _ => {}
        }
        if let Some(u) = update {
            eval_expr(ctx, &scope, u)?;
        }
    }
    Ok(Flow::Normal)
}
