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

use super::ctx::Ctx;
use super::eval_expr::eval_expr;
use super::to_bool::to_bool;

pub fn eval_logical(ctx: &mut Ctx, env: &Env, op: &str, l: &Expr, r: &Expr) -> Result<Value, ()> {
    let a = eval_expr(ctx, env, l)?;
    if op == "&&" {
        if to_bool(&a) {
            eval_expr(ctx, env, r)
        } else {
            Ok(a)
        }
    } else if to_bool(&a) {
        Ok(a)
    } else {
        eval_expr(ctx, env, r)
    }
}
