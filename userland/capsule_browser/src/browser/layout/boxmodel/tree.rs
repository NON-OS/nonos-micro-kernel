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
    // Inline-level on the outside, block on the inside: it sits in a line box
    // like a word but sizes to its own width and height and lays its children
    // in a block context.
    InlineBlock,
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

// Resolved explicit placement of a grid item: zero-based column track and
// row indices with spans, resolved from grid-area names and grid-column
// lines at build time so layout needs no name tables.
#[derive(Clone, Copy)]
pub struct GridPlace {
    pub col: u8,
    pub col_span: u8,
    // None flows the item into the next free row.
    pub row: Option<u8>,
    pub row_span: u8,
}

// One box in the layout tree. Text and Image boxes are leaves; href carries
// the enclosing anchor so hit-testing survives layout, and dom_id ties the
// box back to its DOM node for event dispatch (0 = anonymous).
pub struct BoxNode {
    pub kind: BoxKind,
    pub style: Computed,
    pub href: Option<String>,
    pub dom_id: usize,
    // background-image url captured from the cascade, painted behind content.
    pub bg_image: Option<String>,
    // Explicit grid placement when this box is a grid item that asked for one.
    pub grid_place: Option<GridPlace>,
    pub children: Vec<BoxNode>,
}
