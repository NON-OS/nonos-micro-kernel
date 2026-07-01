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

use crate::browser::css::selector::Selector;
use crate::browser::dom::Dom;

use super::simple::matches_simple;

pub fn matches_selector(dom: &Dom, id: usize, sel: &Selector) -> bool {
    if !matches_simple(dom, id, &sel.key) {
        return false;
    }
    let mut cur = dom.nodes[id].parent;
    for anc in sel.ancestors.iter().rev() {
        loop {
            if cur == 0 {
                return false;
            }
            let parent = dom.nodes[cur].parent;
            let hit = matches_simple(dom, cur, anc);
            cur = parent;
            if hit {
                break;
            }
        }
    }
    true
}
