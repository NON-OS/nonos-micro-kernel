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

use alloc::string::String;
use alloc::vec::Vec;

use super::node::{Node, NodeKind};
use super::tree::Dom;

impl Dom {
    pub fn push(&mut self, parent: usize, kind: NodeKind, tag: String) -> Option<usize> {
        if parent >= self.nodes.len() || self.nodes.len() >= super::limits::MAX_NODES {
            return None;
        }
        let id = self.nodes.len();
        self.nodes.push(Node {
            kind,
            tag,
            text: String::new(),
            attrs: Vec::new(),
            parent,
            children: Vec::new(),
        });
        self.nodes[parent].children.push(id);
        Some(id)
    }
}
