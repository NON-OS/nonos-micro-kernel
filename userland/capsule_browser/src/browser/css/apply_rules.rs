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
use super::matching::matches_selector;
use super::rule::Rule;
use super::rule_index::RuleIndex;
use super::specificity::specificity;

pub(super) fn apply_rules(
    dom: &Dom,
    id: usize,
    rules: &[Rule],
    index: &RuleIndex,
    c: &mut Computed,
    parent_fs: u32,
    vars: &[(String, String)],
    bg: &mut Option<alloc::string::String>,
) {
    let mut hits: Vec<(u32, usize)> = Vec::new();
    // Only rules whose key could match this node; the full matcher still
    // decides, so the applied set is identical to scanning every rule.
    for i in index.candidates(dom, id) {
        let Some(rule) = rules.get(i) else {
            continue;
        };
        let mut best: Option<u32> = None;
        for sel in &rule.selectors {
            // Pseudo-element selectors style generated content, not the host;
            // they cascade separately in pseudo_style.
            if sel.element == 0 && matches_selector(dom, id, sel) {
                let s = specificity(sel);
                best = Some(best.map_or(s, |b| b.max(s)));
            }
        }
        if let Some(s) = best {
            hits.push((s, i));
        }
    }
    // Cascade order: ascending specificity, then source index for ties.
    hits.sort();
    for (_, i) in hits {
        if let Some(rule) = rules.get(i) {
            for d in &rule.decls {
                apply_decl(c, &d.name, &d.value, parent_fs, vars);
                // A later winning background url() overrides an earlier one,
                // matching the cascade order the declarations are applied in.
                if let Some(u) = super::bg_url::bg_url(&d.name, &d.value) {
                    *bg = Some(u);
                }
            }
        }
    }
}
