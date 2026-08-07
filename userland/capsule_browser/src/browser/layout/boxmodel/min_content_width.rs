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

use crate::browser::css::Size;

use super::edges_x::edges_x;
use super::tree::{BoxKind, BoxNode};

const MAX_DEPTH: u32 = 400;
// Same runaway ceiling as the max-content walk.
const CAP: i32 = 8192;

// Min-content width of a box: the narrowest it can be without its content
// overflowing, i.e. the widest unbreakable piece inside it (the longest word
// of any text, or the width of a fixed child). Flex uses this as the floor
// when it shrinks items to fit a crowded row: without it an item collapses
// far below its longest word, wrapping text one token per line into a tall
// column and overrunning its neighbours.
pub(super) fn min_content_width(node: &BoxNode, depth: u32) -> i32 {
    if depth > MAX_DEPTH {
        return 0;
    }
    // A fixed pixel width pins the box; it cannot shrink past it.
    if let Size::Px(p) = node.style.width {
        return (p as i32).clamp(0, CAP);
    }
    let (el, er) = edges_x(&node.style);
    (inner_min(node, depth) + el + er).clamp(0, CAP)
}

// The widest unbreakable child. Blocks, columns and rows all reduce to the
// max child min-content: a row can wrap between items, so its floor is the
// widest single item, not their sum.
fn inner_min(node: &BoxNode, depth: u32) -> i32 {
    match &node.kind {
        BoxKind::Text(t) => widest_word(t, node),
        // An image with no fixed width may scale down freely.
        BoxKind::Image { .. } => 0,
        _ => {
            let mut max = 0i32;
            for c in &node.children {
                max = max.max(min_content_width(c, depth + 1));
            }
            max
        }
    }
}

// The widest single whitespace-separated token, since text only breaks at
// word boundaries.
fn widest_word(t: &str, node: &BoxNode) -> i32 {
    let px = node.style.font_size_px as f32;
    let mut max = 0i32;
    for word in t.split_ascii_whitespace() {
        let w = crate::browser::fonts::measure_text(
            node.style.font_key,
            node.style.mono,
            node.style.bold,
            word,
            px,
            node.style.letter_spacing,
        );
        max = max.max(w);
    }
    max.max(0)
}
