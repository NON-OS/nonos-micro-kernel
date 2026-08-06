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

use crate::browser::js::value::Value;

use super::obj::obj;

// An Error instance: name plus message, tagged so callers can recognise it.
pub fn error_obj(name: &str, message: String) -> Value {
    obj(&[
        ("__error__", Value::Bool(true)),
        ("name", Value::Str(Rc::new(name.into()))),
        ("message", Value::Str(Rc::new(message))),
    ])
}

// Whether a native constructor name is one of the Error family.
pub fn is_err_name(n: &str) -> bool {
    matches!(n, "Error" | "TypeError" | "RangeError" | "SyntaxError" | "ReferenceError")
}
