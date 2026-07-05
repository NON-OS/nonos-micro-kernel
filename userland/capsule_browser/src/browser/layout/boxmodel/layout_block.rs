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

use super::border_box_w::border_box_w;
use super::containing::Containing;
use super::ctx::Ctx;
use super::display_list::{Content, DisplayList, Fragment};
use super::edges_x::edges_x;
use super::edges_y::edges_y;
use super::fixed_h::fixed_h;
use super::layout_absolutes::layout_absolutes;
use super::layout_children::layout_children;
use super::resolve_min_max_h::resolve_min_max_h;
use super::tree::BoxNode;

pub(super) const MAX_DEPTH: u32 = 400;

// Lay one block-level box with its border-box corner at (x, y) inside
// `avail` px of width. Returns the border-box height.
pub(super) fn layout_block(
    node: &BoxNode,
    x: i32,
    y: i32,
    avail: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    if depth > MAX_DEPTH {
        return 0;
    }
    let s = &node.style;
    let bb_w = border_box_w(s, avail);
    // margin:auto on both sides centres an in-flow block that is narrower
    // than its available width.
    let x = if s.margin_auto_x && bb_w < avail { x + (avail - bb_w) / 2 } else { x };
    let (el, er) = edges_x(s);
    let (et, eb) = edges_y(s);
    let content_w = (bb_w - el - er).max(0);
    // This box's definite border-box height, resolving a percentage against
    // the containing height threaded from the ancestor.
    let def_h = fixed_h(s, ctx.cb.h);
    // A positioned box becomes the containing block for absolute descendants;
    // its padding box spans the border box minus the border widths. Either
    // way children inherit this box's definite content height so their own
    // percentage heights resolve against it.
    let mut inner = ctx;
    inner.cb.h = def_h.map(|fh| (fh - et - eb).max(0));
    if s.position != Position::Static {
        inner.cb = Containing {
            x: x + s.border_left as i32,
            y: y + s.border_top as i32,
            w: bb_w - (s.border_left + s.border_right) as i32,
            h: inner.cb.h,
        };
    }
    if s.overflow_hidden {
        // Horizontal clip always applies; vertical only with a fixed height,
        // since an auto box grows to hold its content anyway.
        let x0 = x + s.border_left as i32;
        let x1 = x + bb_w - s.border_right as i32;
        let (y0, y1) = match def_h {
            Some(fh) => (y + s.border_top as i32, y + fh - s.border_bottom as i32),
            None => (i32::MIN, i32::MAX),
        };
        inner = inner.clipped([x0, y0, x1, y1]);
    }
    // The box's own rect goes in first so children paint over it; its height
    // is patched once the children have been laid.
    let slot = frags.len();
    frags.push(Fragment {
        x,
        y,
        w: bb_w,
        h: 0,
        bg: 0,
        border: [0; 4],
        border_color: 0,
        href: None,
        content: Content::None,
        z: ctx.z,
        clip: ctx.clip,
        fixed: ctx.fixed,
        radius: 0,
        node: node.dom_id,
    });
    let inner_h = layout_children(node, x + el, y + et, content_w, frags, depth, inner);
    layout_absolutes(node, frags, depth, inner);
    let h = match def_h {
        Some(fh) => fh,
        None => inner_h + et + eb,
    };
    let h = resolve_min_max_h(s, h);
    let border_color = if s.border_color != 0 { s.border_color } else { s.color };
    if let Some(f) = frags.get_mut(slot) {
        *f = Fragment {
            x,
            y,
            w: bb_w,
            h,
            bg: s.bg,
            border: [s.border_top, s.border_right, s.border_bottom, s.border_left],
            border_color,
            href: node.href.clone(),
            content: Content::None,
            z: ctx.z,
            clip: ctx.clip,
            fixed: ctx.fixed,
            radius: s.radius,
            node: node.dom_id,
        };
    }
    h
}
