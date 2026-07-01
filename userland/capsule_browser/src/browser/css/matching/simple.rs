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

use crate::browser::css::selector::Simple;
use crate::browser::dom::Dom;

pub fn matches_simple(dom: &Dom, id: usize, s: &Simple) -> bool {
    let node = &dom.nodes[id];
    if let Some(t) = &s.tag {
        if &node.tag != t {
            return false;
        }
    }
    if let Some(want) = &s.id {
        if node.attr("id") != Some(want.as_str()) {
            return false;
        }
    }
    let classes = node.attr("class").unwrap_or("");
    for c in &s.classes {
        if !classes.split_whitespace().any(|x| x == c) {
            return false;
        }
    }
    true
}
