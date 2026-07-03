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

// Whether `id` sits inside root's subtree (excluding root itself).
pub(super) fn in_subtree(dom: &Dom, root: usize, id: usize) -> bool {
    let mut cur = id;
    let mut hops = 0u32;
    while cur != 0 && hops < 512 {
        let Some(n) = dom.nodes.get(cur) else {
            return false;
        };
        if n.parent == root {
            return true;
        }
        cur = n.parent;
        hops += 1;
    }
    false
}
