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

use super::tree::Dom;

const MAX_CLONE_DEPTH: u32 = 64;

impl Dom {
    /// Copy `id` and hand back the copy, detached from the document.
    ///
    /// This is how a template becomes a row. A page writes the shape once in
    /// markup, hides it, and every list item is a copy of it filled in, so a
    /// script that cannot copy has to build each row tag by tag or not at
    /// all. The copy is left unattached because the caller decides where it
    /// goes, usually with the same call that would have appended a fresh one.
    ///
    /// Only the tag, text and attributes carry over. Listeners do not, which
    /// matches what a browser does and is also the safe direction: a copy
    /// that inherited handlers would fire them twice.
    pub fn clone_node(&mut self, id: usize, deep: bool) -> Option<usize> {
        let node = self.nodes.get(id)?;
        let (kind, tag) = (node.kind, node.tag.clone());
        let copy = self.create(kind, tag)?;
        self.nodes[copy].text = self.nodes[id].text.clone();
        self.nodes[copy].attrs = self.nodes[id].attrs.clone();
        if deep {
            self.clone_children(id, copy, 0);
        }
        Some(copy)
    }

    /// Copy the subtree under `from` beneath `onto`.
    ///
    /// Depth is capped because the source can be a tree a script built, and a
    /// cycle in it would otherwise be copied forever. Running out of node
    /// budget stops the copy where it is rather than failing the whole call:
    /// a truncated row still renders, and the page keeps running.
    fn clone_children(&mut self, from: usize, onto: usize, depth: u32) {
        if depth >= MAX_CLONE_DEPTH {
            return;
        }
        let kids: Vec<usize> = self.nodes[from].children.clone();
        for kid in kids {
            let Some(copy) = self.clone_node(kid, false) else {
                return;
            };
            if !self.attach(onto, copy) {
                return;
            }
            self.clone_children(kid, copy, depth + 1);
        }
    }
}
