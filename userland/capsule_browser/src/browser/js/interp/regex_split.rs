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

use crate::browser::js::regex::Regex;
use crate::browser::js::value::Value;

use super::regex_obj::{as_regex, pat_of, sub_val};
use super::str_method::str_method;

// String.prototype.split with a regex separator; a plain-string separator is
// delegated to the ordinary string split.
pub fn regex_split(s: &str, argv: &[Value]) -> Value {
    let sep = argv.first().cloned().unwrap_or(Value::Undef);
    if as_regex(&sep).is_none() {
        return str_method(s, "split", argv);
    }
    let (src, flags) = pat_of(&sep);
    let re = Regex::compile(&src, &flags);
    let text: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut last = 0;
    let mut pos = 0;
    while pos <= text.len() {
        match re.find(&text, pos) {
            None => break,
            Some(m) if m.end == m.start => pos = m.start + 1,
            Some(m) => {
                out.push(sub_val(&text, last, m.start));
                last = m.end;
                pos = m.end;
                if out.len() >= 4096 {
                    break;
                }
            }
        }
    }
    out.push(sub_val(&text, last.min(text.len()), text.len()));
    Value::Array(Rc::new(RefCell::new(out)))
}
