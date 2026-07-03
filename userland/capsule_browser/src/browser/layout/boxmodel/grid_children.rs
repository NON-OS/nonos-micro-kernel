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

use super::abs_out_of_flow::out_of_flow;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::layout_box::layout_box;
use super::track_widths::track_widths;
use super::tree::BoxNode;

// Row-major auto placement: item i lands in column i mod n; a full row
// advances by its tallest cell plus the gap. Spans and areas are out of
// scope and every item takes one cell.
pub(super) fn grid_children(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let s = &node.style;
    let cols = track_widths(s, w);
    let n = cols.len().max(1);
    let gap = s.gap as i32;
    let mut cy = 0i32;
    let mut row_h = 0i32;
    let mut col = 0usize;
    for it in &node.children {
        if !it.kind.block_level() || out_of_flow(&it.style) {
            continue;
        }
        if col == n {
            cy += row_h + gap;
            row_h = 0;
            col = 0;
        }
        let Some(&(cx, cw)) = cols.get(col) else {
            break;
        };
        let ml = it.style.margin_left as i32;
        let mr = it.style.margin_right as i32;
        let mt = it.style.margin_top as i32;
        let mb = it.style.margin_bottom as i32;
        let cell_w = (cw - ml - mr).max(1);
        let h = layout_box(it, x + cx + ml, y + cy + mt, cell_w, frags, depth + 1, ctx);
        row_h = row_h.max(h + mt + mb);
        col += 1;
    }
    cy + row_h
}
