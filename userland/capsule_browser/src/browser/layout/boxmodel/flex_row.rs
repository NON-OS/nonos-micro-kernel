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

use crate::browser::css::{Align, Justify, Size};

use super::abs_out_of_flow::out_of_flow;
use super::border_box_w::border_box_w;
use super::content_width::content_width;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::layout_box::layout_box;
use super::min_content_width::min_content_width;
use super::shift_down::shift_down;
use super::tree::BoxNode;

// Every item keeps at least this much main size when the row is crowded.
const MIN_ITEM_W: i32 = 16;

// Row axis: an auto-width item sizes to its content, a fixed item to its
// width. Spare space goes only to items with flex-grow; when items overflow
// the row they shrink proportionally to fit. A measure pass learns heights,
// then each item lays at its justified/aligned slot. Returns the row height.
pub(super) fn flex_row(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let s = &node.style;
    if s.flex_wrap {
        return flex_row_wrapped(node, x, y, w, frags, depth, ctx);
    }
    let gap = s.gap as i32;
    let items: Vec<&BoxNode> =
        node.children.iter().filter(|c| c.kind.block_level() && !out_of_flow(&c.style)).collect();
    let n = items.len() as i32;
    if n == 0 {
        return 0;
    }
    // Base sizes: content width for auto items, the declared width otherwise.
    let mut sizes: Vec<i32> = Vec::with_capacity(items.len());
    let mut weight: Vec<i32> = Vec::with_capacity(items.len());
    let mut margins_gaps = gap * (n - 1);
    let mut base_sum = 0i32;
    let mut total_weight = 0i32;
    for it in &items {
        // flex-basis wins when set; otherwise the width property, otherwise
        // the content width. A zero basis (from "flex: 1") makes grow split
        // the row into equal columns instead of content-proportional ones.
        let g = it.style.flex_grow as i32;
        let b = match it.style.flex_basis {
            Size::Auto => match it.style.width {
                Size::Auto => content_width(it, depth).min(w),
                _ => border_box_w(&it.style, w),
            },
            basis => basis.resolve(w).unwrap_or(0).clamp(0, w),
        };
        margins_gaps += it.style.margin_left as i32 + it.style.margin_right as i32;
        base_sum += b;
        sizes.push(b);
        weight.push(g);
        total_weight += g;
    }
    // Space available to the item boxes themselves (row width minus the gaps
    // and horizontal margins).
    let avail = (w - margins_gaps).max(n * MIN_ITEM_W);
    if base_sum <= avail {
        // Room to spare: grow items with flex-grow absorb it; content-sized
        // items keep their width and any leftover feeds justify-content.
        let free = avail - base_sum;
        if total_weight > 0 && free > 0 {
            let unit = free / total_weight;
            let mut spent = 0;
            let mut last_grow = None;
            for (i, sz) in sizes.iter_mut().enumerate() {
                let add = unit * weight[i];
                *sz += add;
                spent += add;
                if weight[i] > 0 {
                    last_grow = Some(i);
                }
            }
            if let Some(i) = last_grow {
                sizes[i] += free - spent;
            }
        }
    } else if base_sum > 0 {
        // Overflow: shrink every item proportionally so the row fits, but
        // never below its min-content width, or its text collapses into a
        // one-word-per-line column that overruns the next item.
        for (i, sz) in sizes.iter_mut().enumerate() {
            let shrunk = ((*sz as i64 * avail as i64) / base_sum.max(1) as i64) as i32;
            let floor = min_content_width(items[i], depth).max(MIN_ITEM_W);
            *sz = shrunk.max(floor);
        }
    }
    for sz in sizes.iter_mut() {
        *sz = (*sz).max(MIN_ITEM_W).min(w);
    }
    // Leftover main-axis space after sizing. Auto margins on the items absorb
    // it first, so a max-width item with `margin: 0 auto` centres in the row;
    // only when no item carries an auto margin does the leftover feed
    // justify-content.
    let placed: i32 = sizes.iter().sum();
    let leftover = (avail - placed).max(0);
    let auto_count: i32 = items
        .iter()
        .map(|it| it.style.margin_left_auto as i32 + it.style.margin_right_auto as i32)
        .sum();
    let auto_share = if auto_count > 0 { leftover / auto_count } else { 0 };
    let (mut cx, step) = if auto_count > 0 {
        (0, gap)
    } else {
        match s.justify {
            Justify::Start => (0, gap),
            Justify::Center => (leftover / 2, gap),
            Justify::End => (leftover, gap),
            Justify::Between => (0, gap + if n > 1 { leftover / (n - 1) } else { 0 }),
            Justify::Around => {
                let pad = leftover / n;
                (pad / 2, gap + pad)
            }
        }
    };
    // Single layout pass: place each item at its main-axis slot on the row's
    // top edge, recording its fragment range and height. Laying every item out
    // exactly once keeps the whole flex subtree linear.
    let mut row_h = 0i32;
    let mut ranges: Vec<(usize, usize, i32)> = Vec::with_capacity(items.len());
    for (i, it) in items.iter().enumerate() {
        let ml =
            it.style.margin_left as i32 + if it.style.margin_left_auto { auto_share } else { 0 };
        let mr =
            it.style.margin_right as i32 + if it.style.margin_right_auto { auto_share } else { 0 };
        let mt = it.style.margin_top as i32;
        let mb = it.style.margin_bottom as i32;
        let start = frags.len();
        let h = layout_box(it, x + cx + ml, y + mt, sizes[i], frags, depth + 1, ctx);
        let end = frags.len();
        row_h = row_h.max(h + mt + mb);
        ranges.push((start, end, h));
        cx += ml + sizes[i] + mr + step;
    }
    // Cross-axis alignment: shift each item down from the top edge (center or
    // end), or stretch its box to the row height.
    for (i, it) in items.iter().enumerate() {
        let (start, end, h) = ranges[i];
        let mt = it.style.margin_top as i32;
        let mb = it.style.margin_bottom as i32;
        let dy = match s.align {
            Align::Start | Align::Stretch => 0,
            Align::Center => mt.max((row_h - h) / 2) - mt,
            Align::End => (row_h - h - mb).max(0) - mt,
        };
        shift_down(frags, start, end, dy, ctx.clip);
        if matches!(s.align, Align::Stretch) {
            if let Some(f) = frags.get_mut(start) {
                f.h = f.h.max(row_h - mt - mb);
            }
        }
    }
    row_h
}

// flex-wrap: place items left to right, breaking to a fresh line when the
// next item would overrun the content width. Each line advances by its
// tallest item plus the gap. Sized items keep their width; auto items take
// their content width (capped to the row) so they wrap naturally.
fn flex_row_wrapped(
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
    let mut cx = 0i32;
    let mut cy = 0i32;
    let mut line_h = 0i32;
    let mut first_in_line = true;
    for it in &node.children {
        if !it.kind.block_level() || out_of_flow(&it.style) {
            continue;
        }
        let ml = it.style.margin_left as i32;
        let mr = it.style.margin_right as i32;
        let mt = it.style.margin_top as i32;
        let mb = it.style.margin_bottom as i32;
        let iw = match it.style.width {
            Size::Auto => content_width(it, depth).clamp(MIN_ITEM_W, w),
            _ => border_box_w(&it.style, w).clamp(MIN_ITEM_W, w),
        };
        let adv = ml + iw + mr;
        if !first_in_line && cx + gap + adv > w {
            cy += line_h + gap;
            cx = 0;
            line_h = 0;
            first_in_line = true;
        }
        if !first_in_line {
            cx += gap;
        }
        let h = layout_box(it, x + cx + ml, y + cy + mt, iw, frags, depth + 1, ctx);
        line_h = line_h.max(h + mt + mb);
        cx += ml + iw + mr;
        first_in_line = false;
    }
    cy + line_h
}
