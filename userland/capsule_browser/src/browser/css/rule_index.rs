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

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::rule::Rule;

// Rules bucketed by their key (rightmost) simple selector so a node only tests
// rules whose key could match it. Attrs and ancestor combinators are still
// checked in full by matches_selector; the index only prunes candidates and
// never drops a rule that could match.
pub(super) struct RuleIndex {
    by_id: BTreeMap<String, Vec<usize>>,
    by_class: BTreeMap<String, Vec<usize>>,
    by_tag: BTreeMap<String, Vec<usize>>,
    // Universal or attribute-only keys have no id/class/tag entry point, so
    // every node must consider them.
    universal: Vec<usize>,
}

impl RuleIndex {
    pub(super) fn build(rules: &[Rule]) -> RuleIndex {
        let mut idx = RuleIndex {
            by_id: BTreeMap::new(),
            by_class: BTreeMap::new(),
            by_tag: BTreeMap::new(),
            universal: Vec::new(),
        };
        for (i, rule) in rules.iter().enumerate() {
            for sel in &rule.selectors {
                let key = &sel.key;
                if let Some(id) = &key.id {
                    idx.by_id.entry(id.clone()).or_default().push(i);
                } else if !key.classes.is_empty() {
                    // Any of the key's classes is a valid entry point since a
                    // node matching the key carries all of them.
                    for c in &key.classes {
                        idx.by_class.entry(c.clone()).or_default().push(i);
                    }
                } else if let Some(tag) = &key.tag {
                    idx.by_tag.entry(tag.clone()).or_default().push(i);
                } else {
                    idx.universal.push(i);
                }
            }
        }
        idx
    }

    // Rule indices whose key could match this node. A superset of the real
    // matches: matches_selector still decides. Sorted and deduped so each rule
    // is tested at most once.
    pub(super) fn candidates(&self, dom: &Dom, id: usize) -> Vec<usize> {
        let mut out: Vec<usize> = Vec::new();
        let Some(node) = dom.nodes.get(id) else {
            return out;
        };
        if let Some(want) = node.attr("id") {
            if let Some(v) = self.by_id.get(want) {
                out.extend_from_slice(v);
            }
        }
        if let Some(classes) = node.attr("class") {
            for c in classes.split_whitespace() {
                if let Some(v) = self.by_class.get(c) {
                    out.extend_from_slice(v);
                }
            }
        }
        if let Some(v) = self.by_tag.get(node.tag.as_str()) {
            out.extend_from_slice(v);
        }
        out.extend_from_slice(&self.universal);
        out.sort_unstable();
        out.dedup();
        out
    }
}
