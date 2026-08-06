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
use alloc::vec;
use alloc::vec::Vec;

use super::node::{Node, NodeKind};

pub struct Dom {
    pub nodes: Vec<Node>,
    /// The address this document was loaded from.
    ///
    /// It belongs to the document rather than to the viewer: it is what
    /// `location` reports, and what a relative `href` or `src` resolves
    /// against. A script told the page came from somewhere else builds links
    /// that go somewhere else.
    pub base: String,
    /// Where each node was last laid out, as `[x, y, w, h]`.
    ///
    /// A page measures itself to decide what fits, and every one of those
    /// reads answered zero, which is indistinguishable from an element with
    /// no size. The numbers exist in the display list already; they are
    /// copied here because that is what a script can reach.
    pub rects: Vec<[i32; 4]>,
}

impl Dom {
    pub fn new() -> Self {
        let root = Node {
            kind: NodeKind::Document,
            tag: String::new(),
            text: String::new(),
            attrs: Vec::new(),
            parent: 0,
            children: Vec::new(),
        };
        Dom { nodes: vec![root], base: String::new(), rects: Vec::new() }
    }
}

impl Default for Dom {
    /// A document holding nothing but its root, which is what `new` builds.
    fn default() -> Self {
        Self::new()
    }
}
