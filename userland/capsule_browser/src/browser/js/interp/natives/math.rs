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

use super::super::to_num::to_num;
use super::floor::floor;

pub fn math(name: &str, argv: &[Value]) -> Value {
    let a = argv.first().map(to_num).unwrap_or(f64::NAN);
    match name {
        "Math.floor" => Value::Num(floor(a)),
        "Math.round" => Value::Num(floor(a + 0.5)),
        "Math.abs" => Value::Num(if a < 0.0 { -a } else { a }),
        "Math.max" => {
            Value::Num(
                argv.iter().map(to_num).fold(f64::NEG_INFINITY, |x, y| if y > x { y } else { x }),
            )
        }
        "Math.min" => {
            Value::Num(
                argv.iter().map(to_num).fold(f64::INFINITY, |x, y| if y < x { y } else { x }),
            )
        }
        _ => Value::Undef,
    }
}
