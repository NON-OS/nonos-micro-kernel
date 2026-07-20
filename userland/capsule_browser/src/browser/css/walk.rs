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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::apply_rules::apply_rules;
use super::apply_style_attr::apply_style_attr;
use super::budget::MatchBudget;
use super::computed::Computed;
use super::grid_spec::GridSpec;
use super::pseudo_style::{pseudo_style, PseudoText};
use super::rule::Rule;
use super::rule_index::RuleIndex;

type Pseudos = Vec<(Option<PseudoText>, Option<PseudoText>)>;

#[allow(clippy::too_many_arguments)]
pub(super) fn walk(
    dom: &Dom,
    id: usize,
    inherited: Computed,
    ua: &[Rule],
    ua_index: &RuleIndex,
    author: &[Rule],
    author_index: &RuleIndex,
    vars: &[(String, String)],
    styles: &mut Vec<Computed>,
    bg_images: &mut Vec<Option<String>>,
    grids: &mut Vec<Option<GridSpec>>,
    mut pseudos: Option<&mut Pseudos>,
    budget: &mut MatchBudget,
    depth: u32,
) {
    let node = &dom.nodes[id];
    // Box properties reset per element; text properties carry down.
    let mut c = Computed::inherit_from(&inherited);
    let parent_fs = inherited.font_size_px;
    let mut bg: Option<String> = None;
    let mut grid: Option<GridSpec> = None;
    if node.kind == NodeKind::Element {
        apply_rules(dom, id, ua, ua_index, &mut c, parent_fs, vars, &mut bg, &mut grid, None);
        apply_rules(
            dom,
            id,
            author,
            author_index,
            &mut c,
            parent_fs,
            vars,
            &mut bg,
            &mut grid,
            Some(budget),
        );
        if let Some(st) = node.attr("style") {
            apply_style_attr(st, &mut c, parent_fs, vars, &mut bg, &mut grid);
        }
        // Generated content cascades against the element's final style, so
        // the pseudo boxes inherit color and font exactly like a real child.
        if let Some(p) = pseudos.as_deref_mut() {
            p[id] = (
                pseudo_style(dom, id, author, author_index, 1, &c, vars, budget),
                pseudo_style(dom, id, author, author_index, 2, &c, vars, budget),
            );
        }
    }
    styles[id] = c;
    bg_images[id] = bg;
    grids[id] = grid;
    if depth >= 400 {
        return;
    }
    for &ch in &node.children {
        walk(
            dom,
            ch,
            c,
            ua,
            ua_index,
            author,
            author_index,
            vars,
            styles,
            bg_images,
            grids,
            pseudos.as_deref_mut(),
            budget,
            depth + 1,
        );
    }
}
