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
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::regex::{Match, Regex};
use crate::browser::js::value::Value;

use super::regex_obj::sub_val;
use super::to_str::to_str;

// RegExp.prototype.test / exec against argv[0]. A global regex advances its
// lastIndex between calls so exec walks successive matches.
pub fn regex_method(recv: &Value, src: &str, flags: &str, method: &str, argv: &[Value]) -> Value {
    let text: Vec<char> = argv.first().map(to_str).unwrap_or_default().chars().collect();
    let re = Regex::compile(src, flags);
    let start = if re.global { last_index(recv) } else { 0 };
    let m = if start <= text.len() { re.find(&text, start) } else { None };
    match method {
        "test" => {
            if re.global {
                set_last(recv, m.as_ref().map(|x| x.end).unwrap_or(0));
            }
            Value::Bool(m.is_some())
        }
        "exec" => match m {
            None => {
                if re.global {
                    set_last(recv, 0);
                }
                Value::Null
            }
            Some(mm) => {
                if re.global {
                    let next = if mm.end > mm.start { mm.end } else { mm.end + 1 };
                    set_last(recv, next);
                }
                match_array(&text, &mm)
            }
        },
        _ => Value::Undef,
    }
}

// [fullMatch, group1, group2, ...] as a JS array, undefined for unset groups.
pub(super) fn match_array(text: &[char], m: &Match) -> Value {
    let mut out = Vec::new();
    out.push(sub_val(text, m.start, m.end));
    for g in &m.groups {
        out.push(g.map(|(s, e)| sub_val(text, s, e)).unwrap_or(Value::Undef));
    }
    Value::Array(Rc::new(RefCell::new(out)))
}

fn last_index(recv: &Value) -> usize {
    if let Value::Object(m) = recv {
        if let Some(Value::Num(n)) = m.borrow().get("__last__") {
            return *n as usize;
        }
    }
    0
}

fn set_last(recv: &Value, v: usize) {
    if let Value::Object(m) = recv {
        m.borrow_mut().insert("__last__".into(), Value::Num(v as f64));
    }
}
