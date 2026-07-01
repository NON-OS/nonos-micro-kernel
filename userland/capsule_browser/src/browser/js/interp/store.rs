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
use super::set_node_prop::set_node_prop;
use super::to_num::to_num;
use super::to_str::to_str;

pub fn store(ctx: &mut Ctx, env: &Env, target: &Expr, v: Value) -> Result<(), ()> {
    match target {
        Expr::Ident(name) => env.set(name, v),
        Expr::Member(obj, name) => match eval_expr(ctx, env, obj)? {
            Value::Object(map) => {
                map.borrow_mut().insert(name.clone(), v);
            }
            Value::Node(id) => set_node_prop(ctx, id, name, &v),
            _ => {}
        },
        Expr::Index(obj, idx) => {
            let recv = eval_expr(ctx, env, obj)?;
            let key = eval_expr(ctx, env, idx)?;
            match recv {
                Value::Array(a) => {
                    let i = to_num(&key);
                    if (0.0..100_000.0).contains(&i) {
                        let i = i as usize;
                        let mut b = a.borrow_mut();
                        while b.len() <= i {
                            b.push(Value::Undef);
                        }
                        b[i] = v;
                    }
                }
                Value::Object(map) => {
                    map.borrow_mut().insert(to_str(&key), v);
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
