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
use alloc::vec;
use alloc::vec::Vec;

use crate::browser::js::regex::Match;
use crate::browser::js::value::Value;

use super::regex_obj::sub_val;

// Expand a replacement template: `$$` is a literal `$`, `$&` the whole match,
// and `$1`..`$9` the capture groups.
pub(super) fn expand(repl: &str, text: &[char], m: &Match) -> String {
    let rc: Vec<char> = repl.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < rc.len() {
        if rc[i] == '$' && i + 1 < rc.len() {
            let n = rc[i + 1];
            if n == '$' {
                out.push('$');
                i += 2;
                continue;
            }
            if n == '&' {
                out.extend(text[m.start..m.end].iter());
                i += 2;
                continue;
            }
            if let Some(d) = n.to_digit(10) {
                let gi = d as usize;
                if gi >= 1 && gi <= m.groups.len() {
                    if let Some((s, e)) = m.groups[gi - 1] {
                        out.extend(text[s..e].iter());
                    }
                    i += 2;
                    continue;
                }
            }
        }
        out.push(rc[i]);
        i += 1;
    }
    out
}

// Arguments passed to a function replacer: (match, ...groups, offset, whole).
pub(super) fn fn_args(text: &[char], m: &Match) -> Vec<Value> {
    let mut a = vec![sub_val(text, m.start, m.end)];
    for g in &m.groups {
        a.push(g.map(|(s, e)| sub_val(text, s, e)).unwrap_or(Value::Undef));
    }
    a.push(Value::Num(m.start as f64));
    a.push(Value::Str(Rc::new(text.iter().collect())));
    a
}
