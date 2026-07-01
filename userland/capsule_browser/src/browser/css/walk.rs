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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::apply_rules::apply_rules;
use super::apply_style_attr::apply_style_attr;
use super::computed::Computed;
use super::rule::Rule;

pub fn walk(dom: &Dom, id: usize, inherited: Computed, ua: &[Rule], author: &[Rule], styles: &mut Vec<Computed>, depth: u32) {
    let node = &dom.nodes[id];
    let mut c = inherited;
    c.display_none = false;
    if node.kind == NodeKind::Element {
        apply_rules(dom, id, ua, &mut c);
        apply_rules(dom, id, author, &mut c);
        if let Some(st) = node.attr("style") {
            apply_style_attr(st, &mut c);
        }
    }
    styles[id] = c;
    if depth >= 400 {
        return;
    }
    for &ch in &node.children {
        walk(dom, ch, c, ua, author, styles, depth + 1);
    }
}
