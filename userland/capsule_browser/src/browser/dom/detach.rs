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

impl Dom {
    // Unlink a node from its parent; the node and its subtree stay in the
    // arena but nothing walks to them anymore.
    pub fn detach(&mut self, id: usize) {
        if id == 0 || id >= self.nodes.len() {
            return;
        }
        let parent = self.nodes[id].parent;
        if let Some(p) = self.nodes.get_mut(parent) {
            p.children.retain(|&c| c != id);
        }
        self.nodes[id].parent = 0;
    }
}
