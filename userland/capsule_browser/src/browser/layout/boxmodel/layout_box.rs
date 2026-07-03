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

use crate::browser::css::Position;

use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::layout_block::layout_block;
use super::layout_flex::layout_flex;
use super::layout_grid::layout_grid;
use super::rel_offset::rel_offset;
use super::tree::{BoxKind, BoxNode};

// Route a block-level box to its formatting context. A z-index opens a new
// stacking level. position:relative draws the box shifted while its returned
// flow height keeps siblings where normal flow puts them.
pub(super) fn layout_box(
    node: &BoxNode,
    x: i32,
    y: i32,
    avail: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let mut ctx = ctx;
    if node.style.z != 0 {
        ctx.z = node.style.z;
    }
    let (mut x, mut y) = (x, y);
    if node.style.position == Position::Relative {
        let (dx, dy) = rel_offset(&node.style, ctx.cb.w);
        x += dx;
        y += dy;
    }
    match node.kind {
        BoxKind::Flex => layout_flex(node, x, y, avail, frags, depth, ctx),
        BoxKind::Grid => layout_grid(node, x, y, avail, frags, depth, ctx),
        _ => layout_block(node, x, y, avail, frags, depth, ctx),
    }
}
