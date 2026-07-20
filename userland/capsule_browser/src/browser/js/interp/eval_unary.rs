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
use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::eval_expr::eval_expr;
use super::promise_await::await_value;
use super::to_bool::to_bool;
use super::to_num::to_num;
use super::type_of::type_of;

pub fn eval_unary(ctx: &mut Ctx, env: &Env, op: &str, a: &Expr) -> Result<Value, ()> {
    let v = eval_expr(ctx, env, a)?;
    Ok(match op {
        "-" => Value::Num(-to_num(&v)),
        "!" => Value::Bool(!to_bool(&v)),
        "typeof" => Value::Str(Rc::new(type_of(&v))),
        "await" => await_value(ctx, v),
        _ => Value::Undef,
    })
}
