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

use crate::browser::js::value::Value;

use super::to_num::to_num;

pub fn rel(a: &Value, b: &Value, op: &str) -> bool {
    if let (Value::Str(x), Value::Str(y)) = (a, b) {
        let (x, y) = (x.as_str(), y.as_str());
        return match op {
            "<" => x < y,
            ">" => x > y,
            "<=" => x <= y,
            _ => x >= y,
        };
    }
    let (x, y) = (to_num(a), to_num(b));
    if x != x || y != y {
        return false;
    }
    match op {
        "<" => x < y,
        ">" => x > y,
        "<=" => x <= y,
        _ => x >= y,
    }
}
