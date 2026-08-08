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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::matching::matches_selector;
use super::parse::parse_selectors;

/// How far up a parent chain a walk goes before giving up. A tree a script
/// built can hold a cycle, and a walk up it would otherwise never end.
const MAX_ANCESTRY: u32 = 512;

/// Whether one node matches a selector list.
///
/// `select` answers this by walking the whole document and keeping the hits,
/// which is the wrong shape for a script asking about the node it already
/// has. Scripts ask constantly: event delegation is a `closest` call per
/// event, so the walk would run once per click over every node in the page.
pub fn matches(dom: &Dom, id: usize, selector: &str) -> bool {
    if dom.nodes.get(id).map(|n| n.kind) != Some(NodeKind::Element) {
        return false;
    }
    parse_selectors(selector).iter().any(|s| matches_selector(dom, id, s))
}

/// The nearest node at or above `id` that matches, or none.
///
/// This is how a page turns a click on whatever was under the pointer into
/// the row, button or link the handler is about, so it runs on every event a
/// delegating listener sees.
pub fn closest(dom: &Dom, id: usize, selector: &str) -> Option<usize> {
    let sels = parse_selectors(selector);
    if sels.is_empty() {
        return None;
    }
    let mut at = id;
    for _ in 0..MAX_ANCESTRY {
        let node = dom.nodes.get(at)?;
        if node.kind == NodeKind::Element && sels.iter().any(|s| matches_selector(dom, at, s)) {
            return Some(at);
        }
        if node.parent == at {
            return None;
        }
        at = node.parent;
    }
    None
}
