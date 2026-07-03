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

use crate::browser::dom::Dom;

const MAX_GRAFT_DEPTH: u32 = 64;

// Copy src's children under dst_id in dst, subtree by subtree. push()
// enforces the node budget, so a huge fragment truncates instead of growing
// without bound.
pub(super) fn copy_children(dst: &mut Dom, src: &Dom, src_id: usize, dst_id: usize, depth: u32) {
    if depth > MAX_GRAFT_DEPTH {
        return;
    }
    let Some(src_node) = src.nodes.get(src_id) else {
        return;
    };
    for &c in src_node.children.clone().iter() {
        let Some(child) = src.nodes.get(c) else {
            continue;
        };
        let Some(new_id) = dst.push(dst_id, child.kind, child.tag.clone()) else {
            return;
        };
        dst.nodes[new_id].text = child.text.clone();
        dst.nodes[new_id].attrs = child.attrs.clone();
        copy_children(dst, src, c, new_id, depth + 1);
    }
}
