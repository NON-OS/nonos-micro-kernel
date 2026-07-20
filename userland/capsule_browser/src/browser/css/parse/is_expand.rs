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
use alloc::vec;
use alloc::vec::Vec;

// Split a selector list on top-level commas only, so :is(.a,.b) stays whole.
pub(super) fn split_top_level(list: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, ch) in list.char_indices() {
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth <= 0 => {
                out.push(&list[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        if out.len() >= 64 {
            return out;
        }
    }
    out.push(&list[start..]);
    out
}

// Expand :is(...) and :where(...) groups into plain alternatives: the rule
// `:is(.a,.b) .c` becomes `.a .c` and `.b .c`. Each expansion replaces the
// first group and recurses until none remain, bounded so a hostile selector
// cannot blow up the rule set. :where matches identically (its zero
// specificity is not modeled).
pub(super) fn expand_is(part: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut work: Vec<String> = vec![part.to_string()];
    while let Some(cur) = work.pop() {
        if out.len() + work.len() >= 24 {
            out.push(cur);
            continue;
        }
        let Some((pre, args, post)) = first_group(&cur) else {
            out.push(cur);
            continue;
        };
        for alt in split_top_level(args).into_iter().take(8) {
            let alt = alt.trim();
            if alt.is_empty() {
                continue;
            }
            let mut s = String::with_capacity(pre.len() + alt.len() + post.len());
            s.push_str(pre);
            s.push_str(alt);
            s.push_str(post);
            work.push(s);
        }
    }
    out
}

// The first :is( or :where( group: (before, arguments, after).
fn first_group(s: &str) -> Option<(&str, &str, &str)> {
    let (at, open) = match (s.find(":is("), s.find(":where(")) {
        (Some(a), Some(b)) if b < a => (b, b + 7),
        (Some(a), _) => (a, a + 4),
        (None, Some(b)) => (b, b + 7),
        (None, None) => return None,
    };
    let mut depth = 1i32;
    for (i, ch) in s[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some((&s[..at], &s[open..open + i], &s[open + i + 1..]));
                }
            }
            _ => {}
        }
    }
    None
}
