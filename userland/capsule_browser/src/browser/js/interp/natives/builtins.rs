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

use crate::browser::js::value::Value;

use super::super::to_bool::to_bool;
use super::super::to_num::to_num;
use super::super::to_str::to_str;

pub fn builtin(name: &str, argv: &[Value]) -> Value {
    let first = argv.first().cloned().unwrap_or(Value::Undef);
    match name {
        "parseInt" => Value::Num(to_num(&first) as i64 as f64),
        "parseFloat" | "Number" => Value::Num(to_num(&first)),
        "String" => Value::Str(Rc::new(to_str(&first))),
        "Boolean" => Value::Bool(to_bool(&first)),
        "isNaN" => {
            let n = to_num(&first);
            Value::Bool(n.is_nan())
        }
        _ => Value::Undef,
    }
}
