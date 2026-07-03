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
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::value::Value;

use super::to_num::to_num;
use super::to_str::to_str;

// The string methods real pages lean on. Indexes work on characters, not
// bytes, so multi-byte text cannot split mid-sequence.
pub(super) fn str_method(s: &str, method: &str, argv: &[Value]) -> Value {
    let arg = |i: usize| argv.get(i).map(to_str).unwrap_or_default();
    match method {
        "includes" => Value::Bool(s.contains(arg(0).as_str())),
        "startsWith" => Value::Bool(s.starts_with(arg(0).as_str())),
        "endsWith" => Value::Bool(s.ends_with(arg(0).as_str())),
        "indexOf" => {
            let needle = arg(0);
            match s.find(needle.as_str()) {
                Some(byte) => Value::Num(s[..byte].chars().count() as f64),
                None => Value::Num(-1.0),
            }
        }
        "trim" => Value::Str(Rc::new(s.trim().to_string())),
        "toLowerCase" => Value::Str(Rc::new(s.to_lowercase())),
        "toUpperCase" => Value::Str(Rc::new(s.to_uppercase())),
        "charAt" => {
            let i = argv.first().map(|v| to_num(v) as i64).unwrap_or(0);
            let ch = if i >= 0 { s.chars().nth(i as usize) } else { None };
            Value::Str(Rc::new(ch.map(|c| c.to_string()).unwrap_or_default()))
        }
        "charCodeAt" => {
            let i = argv.first().map(|v| to_num(v) as i64).unwrap_or(0);
            let ch = if i >= 0 { s.chars().nth(i as usize) } else { None };
            match ch {
                Some(c) => Value::Num(c as u32 as f64),
                None => Value::Num(f64::NAN),
            }
        }
        "slice" | "substring" => {
            let chars: Vec<char> = s.chars().collect();
            let len = chars.len() as i64;
            let clampi = |v: f64, neg_wraps: bool| -> usize {
                let i = v as i64;
                let i = if i < 0 && neg_wraps { len + i } else { i };
                i.clamp(0, len) as usize
            };
            let neg = method == "slice";
            let a = argv.first().map(|v| clampi(to_num(v), neg)).unwrap_or(0);
            let b = argv.get(1).map(|v| clampi(to_num(v), neg)).unwrap_or(len as usize);
            let (a, b) = if method == "substring" && a > b { (b, a) } else { (a, b) };
            let out: String = if a < b { chars[a..b].iter().collect() } else { String::new() };
            Value::Str(Rc::new(out))
        }
        "split" => {
            let sep = arg(0);
            let parts: Vec<Value> = if sep.is_empty() {
                s.chars().map(|c| Value::Str(Rc::new(c.to_string()))).take(4096).collect()
            } else {
                s.split(sep.as_str())
                    .map(|p| Value::Str(Rc::new(p.to_string())))
                    .take(4096)
                    .collect()
            };
            Value::Array(Rc::new(RefCell::new(parts)))
        }
        "replace" => {
            let from = arg(0);
            if from.is_empty() {
                Value::Str(Rc::new(s.to_string()))
            } else {
                Value::Str(Rc::new(s.replacen(from.as_str(), arg(1).as_str(), 1)))
            }
        }
        "repeat" => {
            let n = argv.first().map(|v| to_num(v) as i64).unwrap_or(0).clamp(0, 1024) as usize;
            if s.len().saturating_mul(n) > 1_048_576 {
                Value::Str(Rc::new(String::new()))
            } else {
                Value::Str(Rc::new(s.repeat(n)))
            }
        }
        _ => Value::Undef,
    }
}
