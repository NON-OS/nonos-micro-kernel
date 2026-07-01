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
use alloc::rc::Rc;

use crate::browser::js::value::Value;

use super::to_num::to_num;
use super::to_str::to_str;

pub fn add(a: &Value, b: &Value) -> Value {
    if matches!(a, Value::Str(_)) || matches!(b, Value::Str(_)) {
        Value::Str(Rc::new(format!("{}{}", to_str(a), to_str(b))))
    } else {
        Value::Num(to_num(a) + to_num(b))
    }
}
