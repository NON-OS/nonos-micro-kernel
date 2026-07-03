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
use alloc::string::String;

use crate::browser::js::ast::Expr;
use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::eval_expr::eval_expr;
use super::to_num::to_num;
use super::to_str::to_str;

pub fn eval_index(ctx: &mut Ctx, env: &Env, obj: &Expr, idx: &Expr) -> Result<Value, ()> {
    let recv = eval_expr(ctx, env, obj)?;
    let key = eval_expr(ctx, env, idx)?;
    Ok(match recv {
        Value::Array(a) => {
            let i = to_num(&key);
            if i >= 0.0 {
                a.borrow().get(i as usize).cloned().unwrap_or(Value::Undef)
            } else {
                Value::Undef
            }
        }
        Value::Object(map) => map.borrow().get(&to_str(&key)).cloned().unwrap_or(Value::Undef),
        Value::Str(s) => {
            let i = to_num(&key);
            if i >= 0.0 {
                s.chars()
                    .nth(i as usize)
                    .map(|c| Value::Str(Rc::new(String::from(c))))
                    .unwrap_or(Value::Undef)
            } else {
                Value::Undef
            }
        }
        _ => Value::Undef,
    })
}
