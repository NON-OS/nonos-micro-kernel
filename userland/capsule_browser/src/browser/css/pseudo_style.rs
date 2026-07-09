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
use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::apply::apply_decl;
use super::computed::Computed;
use super::content_text::content_text;
use super::matching::matches_selector;
use super::rule::Rule;
use super::rule_index::RuleIndex;
use super::specificity::specificity;

// One generated-content box: the decoded content string and its cascaded
// style, inherited from the host element like a real child's would be.
pub struct PseudoText {
    pub text: String,
    pub style: Computed,
}

// Cascade the ::before (which 1) or ::after (which 2) rules matching `id`.
// None when no matching rule declares displayable content, which is also the
// common fast path since most elements have no pseudo rules at all.
pub(super) fn pseudo_style(
    dom: &Dom,
    id: usize,
    rules: &[Rule],
    index: &RuleIndex,
    which: u8,
    host: &Computed,
    vars: &[(String, String)],
) -> Option<PseudoText> {
    let mut hits: Vec<(u32, usize)> = Vec::new();
    for i in index.candidates(dom, id) {
        let Some(rule) = rules.get(i) else { continue };
        let mut best: Option<u32> = None;
        for sel in &rule.selectors {
            if sel.element == which && matches_selector(dom, id, sel) {
                let s = specificity(sel);
                best = Some(best.map_or(s, |b| b.max(s)));
            }
        }
        if let Some(s) = best {
            hits.push((s, i));
        }
    }
    if hits.is_empty() {
        return None;
    }
    hits.sort();
    let mut style = Computed::inherit_from(host);
    let mut text: Option<String> = None;
    for (_, i) in hits {
        let Some(rule) = rules.get(i) else { continue };
        for d in &rule.decls {
            if d.name == "content" {
                text = content_text(&d.value);
            } else {
                apply_decl(&mut style, &d.name, &d.value, host.font_size_px, vars);
            }
        }
    }
    text.map(|text| PseudoText { text, style })
}
