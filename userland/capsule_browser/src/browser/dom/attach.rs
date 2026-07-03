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

use super::tree::Dom;

const MAX_ANCESTRY: u32 = 512;

impl Dom {
    // Reparent `child` under `parent`, refusing self- and cycle-creating
    // moves (a node cannot adopt its own ancestor).
    pub fn attach(&mut self, parent: usize, child: usize) -> bool {
        if child == 0 || parent >= self.nodes.len() || child >= self.nodes.len() || parent == child
        {
            return false;
        }
        let mut cur = parent;
        let mut hops = 0;
        while cur != 0 && hops < MAX_ANCESTRY {
            if cur == child {
                return false;
            }
            cur = match self.nodes.get(cur) {
                Some(n) => n.parent,
                None => return false,
            };
            hops += 1;
        }
        self.detach(child);
        self.nodes[child].parent = parent;
        self.nodes[parent].children.push(child);
        true
    }
}
