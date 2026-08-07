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

use super::node::NodeKind;
use super::tree::Dom;

impl Dom {
    /// Put `child` under `parent`, ahead of `before` when that is a sibling.
    ///
    /// A fragment is not placed. It is a holder a script fills so that
    /// several nodes can go in with one call, and placing the holder itself
    /// would put a node in the document that no markup ever described. Its
    /// children go in instead, in order, and it is left empty behind them.
    pub fn place(&mut self, parent: usize, child: usize, before: usize) -> bool {
        if self.nodes.get(child).map(|n| n.kind) != Some(NodeKind::Document) {
            return self.insert_before(parent, child, before);
        }
        let kids: Vec<usize> = self.nodes[child].children.clone();
        if kids.is_empty() {
            return true;
        }
        let mut placed = false;
        for kid in kids {
            placed |= self.insert_before(parent, kid, before);
        }
        placed
    }
}
