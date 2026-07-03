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

use alloc::string::String;

use crate::browser::js::value::Value;

const MAX_DEPTH: u32 = 32;
const MAX_LEN: usize = 65_536;

// JSON.stringify for the data-shaped values; functions and nodes serialize
// as null like unsupported values do.
pub(super) fn json_stringify(v: &Value, out: &mut String, depth: u32) {
    if depth > MAX_DEPTH || out.len() > MAX_LEN {
        out.push_str("null");
        return;
    }
    match v {
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Num(n) if n.is_finite() => {
            let i = *n as i64;
            if (i as f64) == *n {
                out.push_str(&alloc::format!("{}", i));
            } else {
                out.push_str(&alloc::format!("{}", n));
            }
        }
        Value::Str(s) => {
            out.push('"');
            for ch in s.chars() {
                match ch {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\t' => out.push_str("\\t"),
                    '\r' => out.push_str("\\r"),
                    c if (c as u32) < 0x20 => out.push_str(&alloc::format!("\\u{:04x}", c as u32)),
                    c => out.push(c),
                }
            }
            out.push('"');
        }
        Value::Array(a) => {
            out.push('[');
            for (i, item) in a.borrow().iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                json_stringify(item, out, depth + 1);
            }
            out.push(']');
        }
        Value::Object(map) => {
            out.push('{');
            for (i, (k, item)) in map.borrow().iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                json_stringify(&Value::Str(alloc::rc::Rc::new(k.clone())), out, depth + 1);
                out.push(':');
                json_stringify(item, out, depth + 1);
            }
            out.push('}');
        }
        _ => out.push_str("null"),
    }
}
