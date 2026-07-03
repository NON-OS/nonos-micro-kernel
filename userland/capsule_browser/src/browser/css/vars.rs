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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::rule::Rule;

const MAX_VARS: usize = 2048;
const MAX_DEPTH: u32 = 8;

// Gather every custom property (--name) defined across the sheets. Later
// definitions win, so lookups scan the list from the back. This covers the
// dominant :root global-token case without per-element scoping.
pub(super) fn collect_vars(ua: &[Rule], author: &[Rule]) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for rule in ua.iter().chain(author.iter()) {
        for d in &rule.decls {
            if out.len() >= MAX_VARS {
                return out;
            }
            if d.name.starts_with("--") {
                out.push((d.name.clone(), d.value.trim().to_string()));
            }
        }
    }
    out
}

// Substitute every var(--name) / var(--name, fallback) in `value` with the
// token's definition (or the fallback when undefined), recursing since tokens
// nest. Returns the value unchanged when it holds no var().
pub(super) fn resolve(value: &str, vars: &[(String, String)], depth: u32) -> String {
    if depth >= MAX_DEPTH || !value.contains("var(") {
        return value.to_string();
    }
    let mut out = String::new();
    let mut cursor = 0;
    while let Some(rel) = value[cursor..].find("var(") {
        let p = cursor + rel;
        out.push_str(&value[cursor..p]);
        let inner_start = p + 4;
        match find_close(value, inner_start) {
            Some(close) => {
                let inner = &value[inner_start..close];
                let (name, fallback) = split_top_comma(inner);
                let name = name.trim();
                let sub = match lookup(vars, name) {
                    Some(v) => resolve(v, vars, depth + 1),
                    None => match fallback {
                        Some(f) => resolve(f.trim(), vars, depth + 1),
                        None => String::new(),
                    },
                };
                out.push_str(&sub);
                cursor = close + 1;
            }
            None => {
                // Unbalanced parens: keep the remainder verbatim and stop.
                out.push_str(&value[p..]);
                return out;
            }
        }
    }
    out.push_str(&value[cursor..]);
    out
}

// Byte index of the ')' that closes the '(' opened just before `start`,
// tracking nested parens.
fn find_close(s: &str, start: usize) -> Option<usize> {
    let b = s.as_bytes();
    let mut depth = 1u32;
    let mut i = start;
    while i < b.len() {
        match b[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

// Split "name, fallback" at the first top-level comma; parens in a fallback
// keep their own commas out of the split.
fn split_top_comma(inner: &str) -> (&str, Option<&str>) {
    let b = inner.as_bytes();
    let mut depth = 0u32;
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => return (&inner[..i], inner.get(i + 1..)),
            _ => {}
        }
        i += 1;
    }
    (inner, None)
}

// Last definition wins; names compare case-insensitively since values keep
// their original case while definitions are lowercased at parse time.
fn lookup<'a>(vars: &'a [(String, String)], name: &str) -> Option<&'a str> {
    vars.iter().rev().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
}
