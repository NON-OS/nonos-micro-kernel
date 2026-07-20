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

use alloc::vec;

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::apply::apply;
use super::ctx::Ctx;
use super::equals::equals;
use super::map_obj::set_add;
use super::map_ops::store_of;

// Set.prototype methods: add/has/delete/clear/forEach/values.
pub fn set_method(
    ctx: &mut Ctx,
    env: &Env,
    recv: &Value,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    let store = match store_of(recv, "__set__") {
        Some(s) => s,
        None => return Ok(Value::Undef),
    };
    let val = argv.first().cloned().unwrap_or(Value::Undef);
    match method {
        "add" => {
            set_add(&store, val);
            Ok(recv.clone())
        }
        "has" => Ok(Value::Bool(store.borrow().iter().any(|e| equals(e, &val, true)))),
        "delete" => {
            let idx = store.borrow().iter().position(|e| equals(e, &val, true));
            match idx {
                Some(i) => {
                    store.borrow_mut().remove(i);
                    Ok(Value::Bool(true))
                }
                None => Ok(Value::Bool(false)),
            }
        }
        "clear" => {
            store.borrow_mut().clear();
            Ok(Value::Undef)
        }
        "values" | "keys" => Ok(Value::Array(store.clone())),
        "forEach" => {
            let cb = match argv.first() {
                Some(f @ Value::Func(_)) => f.clone(),
                _ => return Ok(Value::Undef),
            };
            let items = store.borrow().clone();
            for v in items {
                apply(ctx, env, cb.clone(), vec![v.clone(), v])?;
            }
            Ok(Value::Undef)
        }
        _ => Ok(Value::Undef),
    }
}
