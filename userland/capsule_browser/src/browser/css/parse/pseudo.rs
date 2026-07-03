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

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::browser::css::selector::Pseudo;

use super::nth::parse_nth;
use super::simple::parse_simple;

// Split the ":a:b(...)" tail of a compound into pseudo tests. Colons inside
// parentheses belong to the argument.
pub(super) fn parse_pseudo_tail(tail: &str, out: &mut Vec<Pseudo>) {
    let b = tail.as_bytes();
    let mut i = 0;
    while i < b.len() && out.len() < 8 {
        while i < b.len() && b[i] == b':' {
            i += 1;
        }
        let start = i;
        let mut depth = 0i32;
        while i < b.len() {
            match b[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                b':' if depth == 0 => break,
                _ => {}
            }
            i += 1;
        }
        if start < i {
            if let Some(p) = classify(&tail[start..i]) {
                out.push(p);
            }
        }
    }
}

// State pseudos cannot hold in a static render and unknown ones must not
// widen a selector, so both fail closed as Never. Pseudos that always hold
// at rest drop out entirely (None).
fn classify(seg: &str) -> Option<Pseudo> {
    let s = seg.trim().to_ascii_lowercase();
    match s.as_str() {
        "first-child" => return Some(Pseudo::FirstChild),
        "last-child" => return Some(Pseudo::LastChild),
        "only-child" => return Some(Pseudo::OnlyChild),
        "first-of-type" => return Some(Pseudo::FirstOfType),
        "last-of-type" => return Some(Pseudo::LastOfType),
        "empty" => return Some(Pseudo::Empty),
        "link" | "enabled" | "optional" | "root" => return None,
        _ => {}
    }
    if let Some(arg) = s.strip_prefix("nth-child(").and_then(|r| r.strip_suffix(')')) {
        return Some(match parse_nth(arg) {
            Some((a, b)) => Pseudo::NthChild(a, b),
            None => Pseudo::Never,
        });
    }
    if let Some(arg) = s.strip_prefix("not(").and_then(|r| r.strip_suffix(')')) {
        // Only a single compound negates; lists and combinators fail closed.
        if !arg.contains(',') && !arg.contains(char::is_whitespace) && !arg.is_empty() {
            return Some(Pseudo::Not(Box::new(parse_simple(arg))));
        }
        return Some(Pseudo::Never);
    }
    Some(Pseudo::Never)
}
