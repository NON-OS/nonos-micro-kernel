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
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::value::Value;

use super::array_util::{clamp_index, MAX_ARRAY};
use super::equals::equals;
use super::to_num::to_num;
use super::to_str::to_str;

// Non-mutating array queries that return a value or a new array.
pub(super) fn array_query(a: &Rc<RefCell<Vec<Value>>>, method: &str, argv: &[Value]) -> Value {
    let b = a.borrow();
    match method {
        "join" => {
            let sep = argv.first().map(to_str).unwrap_or_else(|| String::from(","));
            let parts: Vec<String> = b.iter().map(to_str).collect();
            Value::Str(Rc::new(parts.join(&sep)))
        }
        "indexOf" => {
            let needle = argv.first().cloned().unwrap_or(Value::Undef);
            let idx = b.iter().position(|v| equals(v, &needle, true));
            Value::Num(idx.map(|i| i as f64).unwrap_or(-1.0))
        }
        "lastIndexOf" => {
            let needle = argv.first().cloned().unwrap_or(Value::Undef);
            let idx = b.iter().rposition(|v| equals(v, &needle, true));
            Value::Num(idx.map(|i| i as f64).unwrap_or(-1.0))
        }
        "includes" => {
            let needle = argv.first().cloned().unwrap_or(Value::Undef);
            Value::Bool(b.iter().any(|v| equals(v, &needle, true)))
        }
        "slice" => {
            let s = argv.first().map(|v| clamp_index(to_num(v), b.len())).unwrap_or(0);
            let e = argv.get(1).map(|v| clamp_index(to_num(v), b.len())).unwrap_or(b.len());
            let out: Vec<Value> = if s < e { b[s..e].to_vec() } else { Vec::new() };
            Value::Array(Rc::new(RefCell::new(out)))
        }
        "concat" => {
            let mut out = b.clone();
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
            Value::Array(Rc::new(RefCell::new(out)))
        }
        "flat" => {
            let mut out = Vec::new();
            for v in b.iter() {
                match v {
                    Value::Array(inner) => out.extend(inner.borrow().iter().cloned()),
                    other => out.push(other.clone()),
                }
            }
            Value::Array(Rc::new(RefCell::new(out)))
        }
        _ => Value::Undef,
    }
}
