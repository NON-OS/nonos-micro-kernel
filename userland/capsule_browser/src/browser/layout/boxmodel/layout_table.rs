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

//! Table formatting. Rows (`tr`, gathered through any row groups) are laid into
//! a shared set of column widths computed from the min- and max-content width of
//! the cells in each column, then each row's cells are placed left to right and
//! the row takes the height of its tallest cell. Column spans are not modelled;
//! each cell occupies one column.

use alloc::vec;
use alloc::vec::Vec;

use crate::browser::css::Size;

use super::border_box_w::border_box_w;
use super::content_width::content_width;
use super::ctx::Ctx;
use super::display_list::{Content, DisplayList, Fragment};
use super::edges_x::edges_x;
use super::edges_y::edges_y;
use super::layout_box::layout_box;
use super::min_content_width::min_content_width;
use super::table_columns::column_widths;
use super::tree::BoxNode;

// The table box's own fragment (background, border) at the given size.
fn table_fragment(node: &BoxNode, x: i32, y: i32, w: i32, h: i32, ctx: Ctx) -> Fragment {
    let s = &node.style;
    let border_color = if s.border_color != 0 { s.border_color } else { s.color };
    Fragment {
        x,
        y,
        w,
        h,
        bg: s.bg,
        border: [s.border_top, s.border_right, s.border_bottom, s.border_left],
        border_color,
        href: node.href.clone(),
        content: Content::None,
        z: ctx.z,
        clip: ctx.clip,
        fixed: ctx.fixed,
        sticky: ctx.sticky,
        alpha: ctx.alpha,
        bg_image: node.bg_image.clone(),
        bg_size: s.bg_size,
        bg_repeat: s.bg_repeat,
        shadow: s.shadow,
        radius: s.radius,
        node: node.dom_id,
    }
}

// The rows of a table: direct table-row children, plus the rows inside any
// row-group child (tbody/thead/tfoot), one level deep.
fn rows(table: &BoxNode) -> Vec<&BoxNode> {
    let mut out = Vec::new();
    for child in &table.children {
        if child.style.is_table_row {
            out.push(child);
        } else if !child.style.is_table_cell {
            for gc in &child.children {
                if gc.style.is_table_row {
                    out.push(gc);
                }
            }
        }
    }
    out
}

// The cells of a row.
fn cells(row: &BoxNode) -> impl Iterator<Item = &BoxNode> {
    row.children.iter().filter(|c| c.style.is_table_cell)
}

pub(super) fn layout_table(
    node: &BoxNode,
    x: i32,
    y: i32,
    avail: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let s = &node.style;
    let bb_w = border_box_w(s, avail);
    let (el, er) = edges_x(s);
    let (et, eb) = edges_y(s);
    let content_w = (bb_w - el - er).max(0);

    let rows = rows(node);
    let ncols = rows.iter().map(|r| cells(r).count()).max().unwrap_or(0);
    if ncols == 0 {
        // No table structure resolved: fall back to normal block flow so the
        // markup is not lost.
        return super::layout_block::layout_block(node, x, y, avail, frags, depth, ctx);
    }

    // Per-column min-content and max-content widths across every cell.
    let mut colmin = vec![0i32; ncols];
    let mut colmax = vec![0i32; ncols];
    for row in &rows {
        for (ci, cell) in cells(row).enumerate() {
            let mn = min_content_width(cell, depth + 1);
            let mx = match cell.style.width {
                // Auto cells size to their real max-content (the width the cell
                // wants when nothing wraps), not the min-content: feeding min as
                // the max collapsed every column onto its longest word, so text
                // over-wrapped and the whole table squeezed into a narrow strip.
                Size::Auto => content_width(cell, depth + 1),
                _ => border_box_w(&cell.style, content_w),
            };
            colmin[ci] = colmin[ci].max(mn);
            colmax[ci] = colmax[ci].max(mx.max(mn));
        }
    }
    let widths = column_widths(&colmax, &colmin, content_w);

    // The table box paints behind the cells; its height is patched once the
    // rows are laid.
    let slot = frags.len();
    frags.push(table_fragment(node, x, y, bb_w, 0, ctx));

    let mut cy = y + et;
    for row in &rows {
        let mut cx = x + el;
        let mut row_h = 0;
        for (ci, cell) in cells(row).enumerate() {
            let cw = widths[ci];
            let ch = layout_box(cell, cx, cy, cw, frags, depth + 1, ctx);
            row_h = row_h.max(ch);
            cx += cw;
        }
        cy += row_h;
    }

    let h = (cy - y) + eb;
    if let Some(f) = frags.get_mut(slot) {
        *f = table_fragment(node, x, y, bb_w, h, ctx);
    }
    h
}
