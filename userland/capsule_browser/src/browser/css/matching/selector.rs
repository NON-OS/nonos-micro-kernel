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

// Ancestor steps run nearest-first; a direct step must match the immediate
// parent, a loose one may skip up the chain.
pub fn matches_selector(dom: &Dom, id: usize, sel: &Selector) -> bool {
    if !matches_simple(dom, id, &sel.key) {
        return false;
    }
    let Some(node) = dom.nodes.get(id) else {
        return false;
    };
    let mut cur = node.parent;
    for step in &sel.ancestors {
        if step.direct {
            if cur == 0 || !matches_simple(dom, cur, &step.simple) {
                return false;
            }
            cur = match dom.nodes.get(cur) {
                Some(n) => n.parent,
                None => return false,
            };
        } else {
            loop {
                if cur == 0 {
                    return false;
                }
                let Some(n) = dom.nodes.get(cur) else {
                    return false;
                };
                let parent = n.parent;
                let hit = matches_simple(dom, cur, &step.simple);
                cur = parent;
                if hit {
                    break;
                }
            }
        }
    }
    true
}
