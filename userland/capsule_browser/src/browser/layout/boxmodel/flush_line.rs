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

use alloc::vec::Vec;

use crate::browser::css::TextAlign;

use super::ctx::Ctx;
use super::display_list::{Content, DisplayList, Fragment};
use super::inline_items::InlineItem;

// Emit one finished line box at (x, top) and return the height consumed.
// `pending` holds (line-relative x, item); `line_w` is the used width.
pub(super) fn flush_line(
    frags: &mut DisplayList,
    pending: &mut Vec<(i32, InlineItem)>,
    x: i32,
    top: i32,
    w: i32,
    line_h: i32,
    line_w: i32,
    align: TextAlign,
    ctx: Ctx,
) -> i32 {
    if pending.is_empty() {
        // Blank line from consecutive breaks still takes vertical room.
        return line_h;
    }
    let extra = (w - line_w).max(0);
    let shift = match align {
        TextAlign::Left => 0,
        TextAlign::Center => extra / 2,
        TextAlign::Right => extra,
    };
    for (ix, item) in pending.drain(..) {
        match item {
            InlineItem::Word {
                text,
                px,
                color,
                bg,
                bold,
                mono,
                underline,
                href,
                adv,
                node,
                ..
            } => {
                frags.push(Fragment {
                    x: x + shift + ix,
                    y: top,
                    w: adv,
                    h: line_h,
                    bg,
                    border: [0; 4],
                    border_color: 0,
                    href,
                    content: Content::Text { text, color, px: px as f32, bold, mono, underline },
                    z: ctx.z,
                    clip: ctx.clip,
                    fixed: ctx.fixed,
                    bg_image: None,
                    shadow: None,
                    radius: 0,
                    node,
                });
            }
            InlineItem::Image { src, alt, w: iw, h: ih, href, node, fit } => {
                frags.push(Fragment {
                    x: x + shift + ix,
                    y: top + (line_h - ih).max(0) / 2,
                    w: iw,
                    h: ih,
                    bg: 0,
                    border: [0; 4],
                    border_color: 0,
                    href,
                    content: Content::Image { src, alt, fit },
                    z: ctx.z,
                    clip: ctx.clip,
                    fixed: ctx.fixed,
                    bg_image: None,
                    shadow: None,
                    radius: 0,
                    node,
                });
            }
            InlineItem::Break => {}
        }
    }
    line_h
}
