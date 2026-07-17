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
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::apply::apply;
use super::ctx::Ctx;
use super::to_bool::to_bool;

// Callback array methods: forEach/map/filter/find/findIndex/some/every. The
// callback receives (item, index) and results drive each method's outcome.
pub(super) fn array_iter(
    ctx: &mut Ctx,
    env: &Env,
    a: &Rc<RefCell<Vec<Value>>>,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    let Some(cb) = argv.first().filter(|v| matches!(v, Value::Func(_))) else {
        return Ok(Value::Undef);
    };
    let items = a.borrow().clone();
    let mut out: Vec<Value> = Vec::new();
    for (i, item) in items.into_iter().enumerate() {
        let r = apply(ctx, env, cb.clone(), vec![item.clone(), Value::Num(i as f64)])?;
        match method {
            "map" => out.push(r),
            "filter" => {
                if to_bool(&r) {
                    out.push(item);
                }
            }
            "find" => {
                if to_bool(&r) {
                    return Ok(item);
                }
            }
            "findIndex" => {
                if to_bool(&r) {
                    return Ok(Value::Num(i as f64));
                }
            }
            "some" => {
                if to_bool(&r) {
                    return Ok(Value::Bool(true));
                }
            }
            "every" => {
                if !to_bool(&r) {
                    return Ok(Value::Bool(false));
                }
            }
            _ => {}
        }
        if ctx.steps >= ctx.budget {
            return Err(());
        }
    }
    Ok(match method {
        "map" | "filter" => Value::Array(Rc::new(RefCell::new(out))),
        "findIndex" => Value::Num(-1.0),
        "some" => Value::Bool(false),
        "every" => Value::Bool(true),
        _ => Value::Undef,
    })
}
