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

use crate::browser::css::Align;

use super::abs_out_of_flow::out_of_flow;
use super::border_box_w::border_box_w;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::layout_box::layout_box;
use super::tree::BoxNode;

// Column axis: items stack top to bottom with the gap between them and
// align-items placing them across the width. Heights stay content-sized, so
// justify-content has no free space to distribute here.
pub(super) fn flex_col(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let s = &node.style;
    let gap = s.gap as i32;
    let mut cy = 0i32;
    let mut first = true;
    for it in &node.children {
        if !it.kind.block_level() || out_of_flow(&it.style) {
            continue;
        }
        if !first {
            cy += gap;
        }
        first = false;
        let ml = it.style.margin_left as i32;
        let mr = it.style.margin_right as i32;
        cy += it.style.margin_top as i32;
        let item_avail = (w - ml - mr).max(0);
        let iw = border_box_w(&it.style, item_avail);
        // Cross-axis auto margins centre the item and override align-items: a
        // narrower `margin: 0 auto` container is centred in the column even
        // when the column stretches or starts its other children.
        // An item with auto margins on both sides centres itself once inside
        // the width its own layout receives, so place it at the content edge
        // and let that single shift happen there. Centring here as well laid
        // the box right of centre by the whole leftover. A left-only auto
        // margin still pushes the item to the end from this side, since the
        // item's own layout ignores single-sided auto margins.
        let dx = if it.style.margin_left_auto && it.style.margin_right_auto {
            ml
        } else if it.style.margin_left_auto {
            (w - iw - mr).max(0)
        } else {
            match s.align {
                Align::Start | Align::Stretch => ml,
                Align::Center => ml.max((w - iw) / 2),
                Align::End => (w - iw - mr).max(0),
            }
        };
        let h = layout_box(it, x + dx, y + cy, item_avail, frags, depth + 1, ctx);
        cy += h + it.style.margin_bottom as i32;
    }
    cy.max(0)
}
