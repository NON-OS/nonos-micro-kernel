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

use crate::browser::css::{Clear, Float, Size};

use super::abs_out_of_flow::out_of_flow;
use super::border_box_w::border_box_w;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::float_ctx::FloatCtx;
use super::layout_box::layout_box;
use super::layout_inline::layout_inline;
use super::min_content_width::min_content_width;
use super::tree::BoxNode;

// Lay a container's in-flow children into its content box at (x, y) by w
// wide. All-inline children run as one inline context; otherwise the
// children are block-level (anonymous wrapping guarantees it) and stack with
// collapsed vertical margins. Absolute children are laid by the container.
pub(super) fn layout_children(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    if node.children.is_empty() {
        return 0;
    }
    let in_flow = |c: &&BoxNode| !out_of_flow(&c.style);
    let all_inline = node.children.iter().filter(in_flow).all(|c| !c.kind.block_level());
    if all_inline {
        return layout_inline(&node.children, x, y, w, &node.style, frags, ctx);
    }
    let mut floats = FloatCtx::new(x, w);
    let mut cy = y;
    let mut prev_mb = 0i32;
    let mut first = true;
    for child in &node.children {
        if !child.kind.block_level() || out_of_flow(&child.style) {
            continue;
        }
        let cs = &child.style;
        let ml = cs.margin_left as i32;
        let mr = cs.margin_right as i32;
        if cs.float != Float::None {
            // A float leaves normal vertical flow: size it, drop it to the row
            // where it fits beside existing floats, and record it so following
            // content wraps around it. The parent's margin run is untouched.
            let is_left = cs.float == Float::Left;
            let clear_y = floats.clear_row(cs.clear, cy);
            let fw = float_width(child, w, depth);
            let outer = fw + ml + mr;
            let (fx, fy) = floats.next_pos(is_left, outer, clear_y);
            let fh = layout_box(child, fx + ml, fy, fw, frags, depth + 1, ctx);
            floats.record(is_left, fx, outer, fy + fh + cs.margin_bottom as i32);
            continue;
        }
        // A cleared box starts below the floats it clears.
        if cs.clear != Clear::None {
            cy = cy.max(floats.clear_row(cs.clear, cy));
        }
        let mt = cs.margin_top as i32;
        cy += if first { mt } else { prev_mb.max(mt) };
        first = false;
        // In-flow content sits in the band left of/right of any active floats.
        let (band_x, band_w) = floats.band(cy);
        let child_avail = (band_w - ml - mr).max(0);
        let ch = layout_box(child, band_x + ml, cy, child_avail, frags, depth + 1, ctx);
        cy += ch;
        prev_mb = cs.margin_bottom as i32;
    }
    cy += prev_mb;
    // The container must be tall enough to contain floats that reach past the
    // in-flow content.
    (cy.max(floats.max_bottom()) - y).max(0)
}

// A float's border-box width: its declared width, or its shrink-to-fit
// min-content width when the width is auto, capped at the container width.
fn float_width(node: &BoxNode, avail: i32, depth: u32) -> i32 {
    match node.style.width {
        Size::Auto => min_content_width(node, depth + 1).min(avail).max(0),
        _ => border_box_w(&node.style, avail),
    }
}
