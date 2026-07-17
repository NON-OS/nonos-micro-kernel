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

use crate::browser::js::env::Env;
use crate::browser::js::regex::Regex;
use crate::browser::js::value::Value;

use super::apply::apply;
use super::ctx::Ctx;
use super::regex_obj::as_regex;
use super::regex_replace_util::{expand, fn_args};
use super::str_method::str_method;
use super::to_str::to_str;

// String.prototype.replace / replaceAll with a regex pattern. The replacement
// is a template string ($1, $&) or a function called per match.
pub fn regex_replace(
    ctx: &mut Ctx,
    env: &Env,
    s: &str,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    let target = argv.first().cloned().unwrap_or(Value::Undef);
    let (src, flags) = match as_regex(&target) {
        Some(pf) => pf,
        None => return Ok(plain_replace(s, method, argv)),
    };
    let global = method == "replaceAll" || flags.contains('g');
    let re = Regex::compile(&src, &flags);
    let text: Vec<char> = s.chars().collect();
    let repl = argv.get(1).cloned().unwrap_or(Value::Undef);
    let mut out = String::new();
    let mut pos = 0;
    while pos <= text.len() {
        let m = match re.find(&text, pos) {
            Some(m) => m,
            None => break,
        };
        out.extend(text[pos..m.start].iter());
        let piece = match &repl {
            Value::Func(_) => to_str(&apply(ctx, env, repl.clone(), fn_args(&text, &m))?),
            other => expand(&to_str(other), &text, &m),
        };
        out.push_str(&piece);
        if m.end > m.start {
            pos = m.end;
        } else {
            if m.start < text.len() {
                out.push(text[m.start]);
            }
            pos = m.start + 1;
        }
        if !global {
            break;
        }
    }
    out.extend(text[pos.min(text.len())..].iter());
    Ok(Value::Str(Rc::new(out)))
}

fn plain_replace(s: &str, method: &str, argv: &[Value]) -> Value {
    if method != "replaceAll" {
        return str_method(s, "replace", argv);
    }
    let from = argv.first().map(to_str).unwrap_or_default();
    let to = argv.get(1).map(to_str).unwrap_or_default();
    if from.is_empty() {
        return Value::Str(Rc::new(s.to_string()));
    }
    Value::Str(Rc::new(s.replace(from.as_str(), to.as_str())))
}
