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
use super::to_str::to_str;

// Build a RegExp value: an object tagged with its source and flags, plus the
// mutable lastIndex used by global matching.
pub fn regex_obj(src: String, flags: String) -> Value {
    obj(&[
        ("__regex__", Value::Bool(true)),
        ("__src__", Value::Str(Rc::new(src))),
        ("__flags__", Value::Str(Rc::new(flags))),
        ("__last__", Value::Num(0.0)),
    ])
}

// If the value is a RegExp, return its (source, flags).
pub fn as_regex(v: &Value) -> Option<(String, String)> {
    if let Value::Object(m) = v {
        let b = m.borrow();
        if b.get("__regex__").is_some() {
            let src = match b.get("__src__") {
                Some(Value::Str(s)) => (**s).clone(),
                _ => String::new(),
            };
            let flags = match b.get("__flags__") {
                Some(Value::Str(s)) => (**s).clone(),
                _ => String::new(),
            };
            return Some((src, flags));
        }
    }
    None
}

// The substring of `text` in [s, e) as a JS string value.
pub fn sub_val(text: &[char], s: usize, e: usize) -> Value {
    Value::Str(Rc::new(text[s..e].iter().collect()))
}

// The (source, flags) to match against: a RegExp arg as-is, or any other value
// coerced to a literal pattern string.
pub fn pat_of(v: &Value) -> (String, String) {
    match as_regex(v) {
        Some(pf) => pf,
        None => (to_str(v), String::new()),
    }
}
