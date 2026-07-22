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

use super::abs_out_of_flow::out_of_flow;
use super::border_box_w::border_box_w;
use super::content_width::content_width;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::layout_box::layout_box;
use super::offset_px::offset_px;
use super::tree::BoxNode;

// Lay this container's absolutely positioned children against the current
// containing block. top/left anchor; right places when left is auto. An auto
// width spans the gap only when both left and right are pinned; with just one
// side, or neither, the box shrinks to its content, which is how a badge or a
// close button anchored to a corner sizes itself.
pub(super) fn layout_absolutes(node: &BoxNode, frags: &mut DisplayList, depth: u32, ctx: Ctx) {
    for c in &node.children {
        if !out_of_flow(&c.style) {
            continue;
        }
        let cb = ctx.cb;
        let left = offset_px(c.style.left, cb.w);
        let right = offset_px(c.style.right, cb.w);
        let bw = match c.style.width {
            Size::Auto if left.is_some() && right.is_some() => {
                (cb.w - left.unwrap_or(0) - right.unwrap_or(0)).max(16)
            }
            Size::Auto => {
                let avail = (cb.w - left.unwrap_or(0) - right.unwrap_or(0)).max(0);
                content_width(c, depth + 1).clamp(16, avail.max(16))
            }
            _ => border_box_w(&c.style, cb.w),
        };
        let x = cb.x
            + match (left, right) {
                (Some(l), _) => l,
                (None, Some(r)) => cb.w - r - bw,
                (None, None) => 0,
            };
        let y = cb.y + offset_px(c.style.top, cb.w).unwrap_or(0);
        layout_box(c, x, y, bw, frags, depth + 1, ctx);
    }
}
