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

use alloc::format;
use alloc::string::{String, ToString};

use crate::browser::js::value::Value;

pub fn to_str(v: &Value) -> String {
    match v {
        Value::Undef => "undefined".to_string(),
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Num(n) => {
            let i = *n as i64;
            if (i as f64) == *n {
                i.to_string()
            } else {
                format!("{}", n)
            }
        }
        Value::Str(s) => (**s).clone(),
        Value::Array(a) => a.borrow().iter().map(to_str).collect::<alloc::vec::Vec<_>>().join(","),
        Value::Object(_) => "[object Object]".to_string(),
        Value::Func(_) | Value::Native(_) => "function".to_string(),
        Value::Node(_) => "[object Node]".to_string(),
        Value::Bound(kind, _) => (*kind).to_string(),
    }
}
