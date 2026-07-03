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

use alloc::vec;
use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::computed::Computed;
use super::parse::parse;
use super::rule::Rule;
use super::rule_index::RuleIndex;
use super::ua::ua_rules;
use super::vars::collect_vars;
use super::walk::walk;

pub fn compute(dom: &Dom, author_css: &str) -> Vec<Computed> {
    let author = parse(author_css);
    let limit = super::budget::rule_limit(dom.nodes.len(), author.len());
    let author = author.get(..limit).unwrap_or(&author[..]);
    cascade(dom, author)
}

// Cascade an already-parsed, already-budgeted author rule set over the tree.
// Shared by the uncached path and the cached relayout path.
pub(super) fn cascade(dom: &Dom, author: &[Rule]) -> Vec<Computed> {
    let ua = ua_rules();
    // Custom properties resolve against a global token table before any
    // per-element parse sees the substituted value.
    let vars = collect_vars(&ua, author);
    let ua_index = RuleIndex::build(&ua);
    let author_index = RuleIndex::build(author);
    let mut styles = vec![Computed::root(); dom.nodes.len()];
    walk(dom, 0, Computed::root(), &ua, &ua_index, author, &author_index, &vars, &mut styles, 0);
    styles
}
