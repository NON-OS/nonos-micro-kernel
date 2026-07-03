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

use super::matching::matches_selector;
use super::parse::parse_selectors;

// Element ids matching a selector list, in document order, capped at
// `limit`. The full matcher runs, so combinators and attributes work.
pub fn select(dom: &Dom, selector: &str, limit: usize) -> Vec<usize> {
    let sels = parse_selectors(selector);
    let mut out: Vec<usize> = Vec::new();
    if sels.is_empty() {
        return out;
    }
    for id in 0..dom.nodes.len() {
        if out.len() >= limit {
            break;
        }
        if dom.nodes[id].kind != NodeKind::Element {
            continue;
        }
        if sels.iter().any(|s| matches_selector(dom, id, s)) {
            out.push(id);
        }
    }
    out
}
