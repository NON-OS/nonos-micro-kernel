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

use crate::browser::css::{Computed, GridSpec, PseudoText};
use crate::browser::dom::node::Node;
use crate::browser::dom::Dom;

// Shared state of one box-tree build walk: the source DOM, the resolved
// styles and the box budget counter.
pub(super) struct Walk<'a, 'b> {
    pub dom: &'a Dom,
    pub styles: &'a [Computed],
    pub bg_images: &'a [Option<String>],
    pub grids: &'a [Option<GridSpec>],
    pub pseudos: &'a [(Option<PseudoText>, Option<PseudoText>)],
    pub count: &'b mut usize,
}

// One element child under consideration: the node, its DOM id, the parent's
// tag and its 1-based li ordinal within that parent.
pub(super) struct ElementIn<'a> {
    pub c: &'a Node,
    pub ch: usize,
    pub parent_tag: &'a str,
    pub ordinal: u32,
}
