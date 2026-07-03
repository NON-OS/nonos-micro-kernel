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
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::apply::apply;
use super::ctx::Ctx;
use super::equals::equals;
use super::to_bool::to_bool;
use super::to_num::to_num;
use super::to_str::to_str;

const MAX_ARRAY: usize = 100_000;

// The array methods real pages lean on. Callback methods snapshot the items
// so the callback mutating the array cannot invalidate the walk.
pub(super) fn array_method(
    ctx: &mut Ctx,
    env: &Env,
    a: &Rc<RefCell<Vec<Value>>>,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    match method {
        "push" => {
            let mut b = a.borrow_mut();
            for v in argv {
                if b.len() >= MAX_ARRAY {
                    break;
                }
                b.push(v.clone());
            }
            Ok(Value::Num(b.len() as f64))
        }
        "pop" => Ok(a.borrow_mut().pop().unwrap_or(Value::Undef)),
        "shift" => {
            let mut b = a.borrow_mut();
            if b.is_empty() {
                Ok(Value::Undef)
            } else {
                Ok(b.remove(0))
            }
        }
        "join" => {
            let sep = argv.first().map(to_str).unwrap_or_else(|| String::from(","));
            let parts: Vec<String> = a.borrow().iter().map(to_str).collect();
            Ok(Value::Str(Rc::new(parts.join(&sep))))
        }
        "indexOf" => {
            let needle = argv.first().cloned().unwrap_or(Value::Undef);
            let idx = a.borrow().iter().position(|v| equals(v, &needle, true));
            Ok(Value::Num(idx.map(|i| i as f64).unwrap_or(-1.0)))
        }
        "includes" => {
            let needle = argv.first().cloned().unwrap_or(Value::Undef);
            Ok(Value::Bool(a.borrow().iter().any(|v| equals(v, &needle, true))))
        }
        "slice" => {
            let b = a.borrow();
            let len = b.len() as i64;
            let clampi = |v: f64| -> usize {
                let i = v as i64;
                let i = if i < 0 { len + i } else { i };
                i.clamp(0, len) as usize
            };
            let s = argv.first().map(|v| clampi(to_num(v))).unwrap_or(0);
            let e = argv.get(1).map(|v| clampi(to_num(v))).unwrap_or(len as usize);
            let out: Vec<Value> = if s < e { b[s..e].to_vec() } else { Vec::new() };
            Ok(Value::Array(Rc::new(RefCell::new(out))))
        }
        "concat" => {
            let mut out = a.borrow().clone();
            for v in argv {
                match v {
                    Value::Array(other) => out.extend(other.borrow().iter().cloned()),
                    other => out.push(other.clone()),
                }
                if out.len() > MAX_ARRAY {
                    out.truncate(MAX_ARRAY);
                    break;
                }
            }
            Ok(Value::Array(Rc::new(RefCell::new(out))))
        }
        "forEach" | "map" | "filter" | "find" => {
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
                    _ => {}
                }
                if ctx.steps >= ctx.budget {
                    return Err(());
                }
            }
            match method {
                "map" | "filter" => Ok(Value::Array(Rc::new(RefCell::new(out)))),
                "find" => Ok(Value::Undef),
                _ => Ok(Value::Undef),
            }
        }
        _ => Ok(Value::Undef),
    }
}
