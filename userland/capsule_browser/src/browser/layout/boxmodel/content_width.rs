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
use super::image_box::image_box;
use super::tree::{BoxKind, BoxNode};

const MAX_DEPTH: u32 = 400;
// A wide ceiling for content that never wraps, capping runaway text.
const CAP: i32 = 8192;

// Max-content width of a flex item: the width it wants when nothing wraps.
// This walks the box tree ONCE measuring text and image extents; it never
// re-enters the layout machine, so flex sizing stays linear instead of the
// exponential cost of laying each item out to probe it.
pub(super) fn content_width(node: &BoxNode, depth: u32) -> i32 {
    if depth > MAX_DEPTH {
        return 0;
    }
    // An explicit pixel width pins the box. A percentage width has no
    // intrinsic size of its own: for max-content measurement it resolves
    // against a containing block we do not have here, so treat it as auto and
    // size to the content. Returning CAP instead would report every percent
    // box as 8192px wide and blow up a flex parent's base sizing, collapsing
    // its siblings to the minimum item width.
    match node.style.width {
        Size::Px(p) => return (p as i32).clamp(0, CAP),
        // A calc with no percent part is as definite as plain pixels.
        Size::Calc(px, 0) => return px.clamp(0, CAP),
        Size::Pct(_) | Size::Calc(_, _) | Size::Auto => {}
    }
    let (el, er) = edges_x(&node.style);
    (inner_width(node, depth) + el + er).clamp(0, CAP)
}

// Content width of a box's children: block/flex-col children stack (widest
// wins), a flex row and inline runs place side by side (sum).
fn inner_width(node: &BoxNode, depth: u32) -> i32 {
    match &node.kind {
        BoxKind::Text(t) => text_width(t, node),
        BoxKind::Image { .. } => image_box(&node.style, CAP).0,
        BoxKind::Flex if !node.style.flex_col => {
            let gap = node.style.gap as i32;
            let mut sum = 0i32;
            let mut n = 0i32;
            for c in &node.children {
                sum = sum.saturating_add(
                    content_width(c, depth + 1)
                        + c.style.margin_left as i32
                        + c.style.margin_right as i32,
                );
                n += 1;
            }
            sum + gap * (n - 1).max(0)
        }
        _ => {
            // Block, flex column and grid stack their block children (max),
            // while contiguous inline children share a line (sum).
            let mut max = 0i32;
            let mut run = 0i32;
            for c in &node.children {
                if c.kind.block_level() {
                    max = max.max(run);
                    run = 0;
                    max = max.max(content_width(c, depth + 1));
                } else {
                    run = run.saturating_add(content_width(c, depth + 1));
                }
            }
            max.max(run)
        }
    }
}

fn text_width(t: &str, node: &BoxNode) -> i32 {
    let s = t.trim();
    if s.is_empty() {
        return 0;
    }
    let px = node.style.font_size_px as f32;
    let w = crate::browser::fonts::measure_text(node.style.font_key, node.style.mono, s, px);
    w.max(0)
}
