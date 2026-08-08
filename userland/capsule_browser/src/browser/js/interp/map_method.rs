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
use super::map_ops::{column, delete_entry, find_index, set_entry, store_of};

// Map.prototype methods: get/set/has/delete/clear/forEach/keys/values/entries.
pub fn map_method(
    ctx: &mut Ctx,
    env: &Env,
    recv: &Value,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    let store = match store_of(recv, "__map__") {
        Some(s) => s,
        None => return Ok(Value::Undef),
    };
    let key = argv.first().cloned().unwrap_or(Value::Undef);
    match method {
        "get" => Ok(find_index(&store, &key)
            .and_then(|i| match &store.borrow()[i] {
                Value::Array(p) => p.borrow().get(1).cloned(),
                _ => None,
            })
            .unwrap_or(Value::Undef)),
        "has" => Ok(Value::Bool(find_index(&store, &key).is_some())),
        "set" => {
            set_entry(&store, key, argv.get(1).cloned().unwrap_or(Value::Undef));
            Ok(recv.clone())
        }
        "delete" => Ok(Value::Bool(delete_entry(&store, &key))),
        "clear" => {
            store.borrow_mut().clear();
            Ok(Value::Undef)
        }
        "keys" => Ok(column(&store, 0)),
        "values" => Ok(column(&store, 1)),
        "entries" => Ok(Value::Array(store.clone())),
        "forEach" => {
            let cb = match argv.first() {
                Some(f @ Value::Func(_)) => f.clone(),
                _ => return Ok(Value::Undef),
            };
            let pairs = store.borrow().clone();
            for e in pairs {
                if let Value::Array(p) = e {
                    let (k, v) = (p.borrow().first().cloned(), p.borrow().get(1).cloned());
                    apply(
                        ctx,
                        env,
                        cb.clone(),
                        vec![v.unwrap_or(Value::Undef), k.unwrap_or(Value::Undef)],
                    )?;
                }
            }
            Ok(Value::Undef)
        }
        _ => Ok(Value::Undef),
    }
}
