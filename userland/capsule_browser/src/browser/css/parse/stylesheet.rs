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

use super::decls::parse_decls;
use super::selectors::parse_selectors;
use super::strip_comments::strip_comments;

pub fn parse(css: &str) -> Vec<Rule> {
    let src = strip_comments(css);
    let mut rules: Vec<Rule> = Vec::new();
    let mut rest = src.as_str();
    while let Some(open) = rest.find('{') {
        let head = rest[..open].trim();
        let after = &rest[open + 1..];
        let close = after.find('}').unwrap_or(after.len());
        if !head.starts_with('@') {
            let selectors = parse_selectors(head);
            let decls = parse_decls(&after[..close]);
            if !selectors.is_empty() && !decls.is_empty() {
                rules.push(Rule { selectors, decls });
            }
        }
        rest = after.get(close + 1..).unwrap_or("");
        if rules.len() >= 4096 {
            break;
        }
    }
    rules
}
