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

use crate::browser::js::env::Env;
use crate::browser::js::regex::Regex;
use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::regex_method::match_array;
use super::regex_obj::{pat_of, sub_val};
use super::regex_replace::regex_replace;
use super::regex_split::regex_split;
use super::str_method::str_method;

// String.prototype.{match,search,replace,replaceAll,split}: the regex-aware
// entry point; non-regex arguments fall back to the plain string methods.
pub fn str_regex_method(
    ctx: &mut Ctx,
    env: &Env,
    s: &str,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    match method {
        "match" => Ok(str_match(s, argv)),
        "search" => Ok(str_search(s, argv)),
        "replace" | "replaceAll" => regex_replace(ctx, env, s, method, argv),
        "split" => Ok(regex_split(s, argv)),
        _ => Ok(str_method(s, method, argv)),
    }
}

fn str_match(s: &str, argv: &[Value]) -> Value {
    let (src, flags) = pat_of(argv.first().unwrap_or(&Value::Undef));
    let re = Regex::compile(&src, &flags);
    let text: Vec<char> = s.chars().collect();
    if !re.global {
        return match re.find(&text, 0) {
            Some(m) => match_array(&text, &m),
            None => Value::Null,
        };
    }
    let mut out = Vec::new();
    let mut pos = 0;
    while let Some(m) = re.find(&text, pos) {
        out.push(sub_val(&text, m.start, m.end));
        pos = if m.end > m.start { m.end } else { m.end + 1 };
        if pos > text.len() || out.len() >= 100_000 {
            break;
        }
    }
    if out.is_empty() {
        Value::Null
    } else {
        Value::Array(Rc::new(RefCell::new(out)))
    }
}

fn str_search(s: &str, argv: &[Value]) -> Value {
    let (src, flags) = pat_of(argv.first().unwrap_or(&Value::Undef));
    let re = Regex::compile(&src, &flags);
    let text: Vec<char> = s.chars().collect();
    match re.find(&text, 0) {
        Some(m) => Value::Num(m.start as f64),
        None => Value::Num(-1.0),
    }
}
