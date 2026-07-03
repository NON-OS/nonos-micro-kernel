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

use alloc::vec::Vec;

use crate::browser::css::rule::Rule;
use crate::browser::manifest::WIDTH;

use super::decls::parse_decls;
use super::matching_brace::matching_brace;
use super::media_matches::media_matches;
use super::selectors::parse_selectors;

const MAX_RULES: usize = 4096;
const MAX_MEDIA_DEPTH: u32 = 4;

// Append the rules found in `src`. @media blocks matching the viewport
// recurse; other at-rules skip their whole block, and statement at-rules
// (@import;) drop off the selector head.
pub(super) fn parse_into(src: &str, rules: &mut Vec<Rule>, depth: u32) {
    let mut rest = src;
    while let Some(open) = rest.find('{') {
        let head = rest[..open].trim();
        let head = head.rsplit(';').next().unwrap_or("").trim();
        let after = &rest[open + 1..];
        if let Some(cond) = head.strip_prefix("@media") {
            let end = matching_brace(after);
            if depth < MAX_MEDIA_DEPTH && media_matches(cond, WIDTH) {
                parse_into(&after[..end], rules, depth + 1);
            }
            rest = after.get(end + 1..).unwrap_or("");
        } else if head.starts_with('@') {
            let end = matching_brace(after);
            rest = after.get(end + 1..).unwrap_or("");
        } else {
            let close = after.find('}').unwrap_or(after.len());
            let selectors = parse_selectors(head);
            let decls = parse_decls(&after[..close]);
            if !selectors.is_empty() && !decls.is_empty() {
                rules.push(Rule { selectors, decls });
            }
            rest = after.get(close + 1..).unwrap_or("");
        }
        if rules.len() >= MAX_RULES {
            break;
        }
    }
}
