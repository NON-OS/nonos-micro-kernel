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

use crate::browser::css::Computed;

use super::collect_items::collect_items;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::flush_line::flush_line;
use super::inline_items::InlineItem;
use super::tree::BoxNode;

// Lay an inline formatting context into line boxes at (x, y), wrapping at w.
// Returns the height consumed.
pub(super) fn layout_inline(
    children: &[BoxNode],
    x: i32,
    y: i32,
    w: i32,
    container: &Computed,
    frags: &mut DisplayList,
    ctx: Ctx,
) -> i32 {
    let mut items: Vec<InlineItem> = Vec::new();
    collect_items(children, w, &mut items, 0);
    if items.is_empty() {
        return 0;
    }
    let base_lh = container.line_height() as i32;
    let align = container.text_align;
    let mut pending: Vec<(i32, InlineItem)> = Vec::new();
    let mut cx = 0i32;
    let mut line_h = 0i32;
    let mut cy = 0i32;
    for item in items {
        if matches!(item, InlineItem::Break) {
            cy +=
                flush_line(frags, &mut pending, x, y + cy, w, line_h.max(base_lh), cx, align, ctx);
            cx = 0;
            line_h = 0;
            continue;
        }
        let adv = item.advance_w();
        let gap = if cx == 0 { 0 } else { item.space_w() };
        if cx > 0 && cx + gap + adv > w {
            cy +=
                flush_line(frags, &mut pending, x, y + cy, w, line_h.max(base_lh), cx, align, ctx);
            cx = 0;
            line_h = 0;
        }
        let gap = if cx == 0 { 0 } else { item.space_w() };
        line_h = line_h.max(item.item_h());
        pending.push((cx + gap, item));
        cx += gap + adv;
    }
    if !pending.is_empty() {
        cy += flush_line(frags, &mut pending, x, y + cy, w, line_h.max(base_lh), cx, align, ctx);
    }
    cy
}
