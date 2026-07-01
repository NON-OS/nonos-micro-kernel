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

use super::add::add;
use super::ctx::Ctx;
use super::equals::equals;
use super::eval_expr::eval_expr;
use super::rel::rel;
use super::to_num::to_num;

pub fn eval_binary(ctx: &mut Ctx, env: &Env, op: &str, l: &Expr, r: &Expr) -> Result<Value, ()> {
    let a = eval_expr(ctx, env, l)?;
    let b = eval_expr(ctx, env, r)?;
    Ok(match op {
        "+" => add(&a, &b),
        "-" => Value::Num(to_num(&a) - to_num(&b)),
        "*" => Value::Num(to_num(&a) * to_num(&b)),
        "/" => Value::Num(to_num(&a) / to_num(&b)),
        "%" => Value::Num(to_num(&a) % to_num(&b)),
        "<" | ">" | "<=" | ">=" => Value::Bool(rel(&a, &b, op)),
        "==" => Value::Bool(equals(&a, &b, false)),
        "!=" => Value::Bool(!equals(&a, &b, false)),
        "===" => Value::Bool(equals(&a, &b, true)),
        "!==" => Value::Bool(!equals(&a, &b, true)),
        _ => Value::Undef,
    })
}
