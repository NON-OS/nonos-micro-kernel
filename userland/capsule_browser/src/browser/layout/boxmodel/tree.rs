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

use crate::browser::css::Computed;

pub enum BoxKind {
    Block,
    Inline,
    Flex,
    Grid,
    Text(String),
    Image { src: String, alt: String },
}

impl BoxKind {
    // Block-level boxes stack in flow and count as flex/grid items.
    pub(super) fn block_level(&self) -> bool {
        matches!(self, BoxKind::Block | BoxKind::Flex | BoxKind::Grid)
    }
}

// One box in the layout tree. Text and Image boxes are leaves; href carries
// the enclosing anchor so hit-testing survives layout, and dom_id ties the
// box back to its DOM node for event dispatch (0 = anonymous).
pub struct BoxNode {
    pub kind: BoxKind,
    pub style: Computed,
    pub href: Option<String>,
    pub dom_id: usize,
    pub children: Vec<BoxNode>,
}
