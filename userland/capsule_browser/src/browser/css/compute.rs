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

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::computed::Computed;
use super::parse::parse;
use super::pseudo_style::PseudoText;
use super::rule::Rule;
use super::rule_index::RuleIndex;
use super::ua::ua_rules;
use super::vars::collect_vars;
use super::walk::walk;

// The cascade output: a computed style per node, and the background image url
// per node so the layout can attach it without carrying a string in the Copy
// style struct.
pub struct Styled {
    pub styles: Vec<Computed>,
    pub bg_images: Vec<Option<String>>,
    // Generated content per node: the cascaded ::before and ::after boxes.
    pub pseudos: Vec<(Option<PseudoText>, Option<PseudoText>)>,
}

pub fn compute(dom: &Dom, author_css: &str) -> Styled {
    let author = parse(author_css);
    let limit = super::budget::rule_limit(dom.nodes.len(), author.len());
    let author = author.get(..limit).unwrap_or(&author[..]);
    cascade(dom, author)
}

// Cascade an already-parsed, already-budgeted author rule set over the tree.
// Shared by the uncached path and the cached relayout path.
pub(super) fn cascade(dom: &Dom, author: &[Rule]) -> Styled {
    let ua = ua_rules();
    // Custom properties resolve against a global token table before any
    // per-element parse sees the substituted value.
    let vars = collect_vars(&ua, author);
    let ua_index = RuleIndex::build(&ua);
    let author_index = RuleIndex::build(author);
    let n = dom.nodes.len();
    let mut styles = vec![Computed::root(); n];
    let mut bg_images = vec![None; n];
    let mut pseudos: Vec<(Option<PseudoText>, Option<PseudoText>)> = Vec::new();
    pseudos.resize_with(n, || (None, None));
    // Pseudo cascading only runs when the sheet declares any pseudo rules.
    let has_pseudos = author.iter().any(|r| r.selectors.iter().any(|s| s.element != 0));
    walk(
        dom,
        0,
        Computed::root(),
        &ua,
        &ua_index,
        author,
        &author_index,
        &vars,
        &mut styles,
        &mut bg_images,
        if has_pseudos { Some(&mut pseudos) } else { None },
        0,
    );
    Styled { styles, bg_images, pseudos }
}
