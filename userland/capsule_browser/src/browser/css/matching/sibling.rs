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

// 1-based position of an element among its element siblings, overall and
// among those sharing its tag: (pos, count, pos_of_type, count_of_type).
pub(super) fn element_position(dom: &Dom, id: usize) -> Option<(i32, i32, i32, i32)> {
    let node = dom.nodes.get(id)?;
    let parent = dom.nodes.get(node.parent)?;
    let mut pos = 0;
    let mut count = 0;
    let mut pos_ty = 0;
    let mut count_ty = 0;
    for &ch in &parent.children {
        let Some(c) = dom.nodes.get(ch) else { continue };
        if c.kind != NodeKind::Element {
            continue;
        }
        count += 1;
        let same_tag = c.tag == node.tag;
        if same_tag {
            count_ty += 1;
        }
        if ch == id {
            pos = count;
            pos_ty = count_ty;
        }
    }
    if pos == 0 {
        return None;
    }
    Some((pos, count, pos_ty, count_ty))
}
