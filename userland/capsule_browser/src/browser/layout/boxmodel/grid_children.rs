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

use super::abs_out_of_flow::out_of_flow;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::layout_box::layout_box;
use super::shift_down::shift_down;
use super::track_widths::track_widths;
use super::tree::BoxNode;

// Rows are implicit, so a hostile placement cannot grow the occupancy table
// past this; items beyond it stack into the last row.
const MAX_ROWS: usize = 512;

// Lay the grid items into the column tracks. Items with a resolved explicit
// placement take their cells first; the rest auto-flow row-major into the
// free cells. Items are laid at the container top and shifted down once the
// row heights are known.
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
    let gap = s.gap as i32;
    let items: Vec<&BoxNode> = node
        .children
        .iter()
        .filter(|it| it.kind.block_level() && !out_of_flow(&it.style))
        .collect();
    // The item count comes first: an auto-fit template sizes its tracks from
    // how many items there are to put in them.
    let cols = track_widths(s, w, items.len());
    let n = cols.len().max(1);
    // Occupancy per row; explicit items reserve their cells first so the
    // auto-flow items fill around them.
    let mut used: Vec<Vec<bool>> = Vec::new();
    let mut slots: Vec<(usize, usize, usize, usize)> = Vec::with_capacity(items.len());
    for it in &items {
        let (row, col, cspan, rspan) = match it.grid_place {
            Some(p) => {
                let col = (p.col as usize).min(n - 1);
                let cspan = (p.col_span as usize).clamp(1, n - col);
                let rspan = (p.row_span as usize).clamp(1, MAX_ROWS);
                let row = match p.row {
                    Some(r) => (r as usize).min(MAX_ROWS - 1),
                    None => free_row(&used, n, col, cspan),
                };
                (row, col, cspan, rspan)
            }
            None => {
                let (row, col) = free_cell(&used, n);
                (row, col, 1, 1)
            }
        };
        reserve(&mut used, n, row, col, cspan, rspan);
        slots.push((row, col, cspan, rspan));
    }
    // Lay every item at the container top, remembering its fragment range so
    // the whole cell can drop to its final row afterwards.
    let mut heights: Vec<i32> = Vec::with_capacity(items.len());
    let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(items.len());
    for (i, it) in items.iter().enumerate() {
        let Some(&(_, col, cspan, _)) = slots.get(i) else {
            break;
        };
        let Some(&(cx, cw)) = cols.get(col) else {
            heights.push(0);
            ranges.push((frags.len(), frags.len()));
            continue;
        };
        // A span covers its tracks plus the gaps between them.
        let mut span_w = cw;
        for k in 1..cspan {
            if let Some(&(_, wk)) = cols.get(col + k) {
                span_w += gap + wk;
            }
        }
        let ml = it.style.margin_left as i32;
        let mr = it.style.margin_right as i32;
        let mt = it.style.margin_top as i32;
        let mb = it.style.margin_bottom as i32;
        let cell_w = (span_w - ml - mr).max(1);
        let start = frags.len();
        let h = layout_box(it, x + cx + ml, y + mt, cell_w, frags, depth + 1, ctx);
        heights.push(h + mt + mb);
        ranges.push((start, frags.len()));
    }
    // Row heights: single-row items set the base; a multi-row item that does
    // not fit its spanned rows grows the last one.
    let nrows = used.len().max(1);
    let mut row_h: Vec<i32> = alloc::vec![0; nrows];
    for (i, &(row, _, _, rspan)) in slots.iter().enumerate() {
        if rspan == 1 {
            if let (Some(rh), Some(h)) = (row_h.get_mut(row), heights.get(i)) {
                *rh = (*rh).max(*h);
            }
        }
    }
    for (i, &(row, _, _, rspan)) in slots.iter().enumerate() {
        if rspan > 1 {
            let end = (row + rspan).min(nrows);
            let mut have = gap * end.saturating_sub(row + 1) as i32;
            for rh in row_h.iter().take(end).skip(row) {
                have += *rh;
            }
            let deficit = heights.get(i).copied().unwrap_or(0) - have;
            if deficit > 0 {
                if let Some(rh) = row_h.get_mut(end.saturating_sub(1)) {
                    *rh += deficit;
                }
            }
        }
    }
    // Prefix offsets, then drop each item's fragments to its row.
    let mut row_y: Vec<i32> = Vec::with_capacity(nrows);
    let mut acc = 0i32;
    for (r, rh) in row_h.iter().enumerate() {
        row_y.push(acc);
        acc += rh + if r + 1 < nrows { gap } else { 0 };
    }
    for (i, &(row, _, _, _)) in slots.iter().enumerate() {
        let dy = row_y.get(row).copied().unwrap_or(0);
        let Some(&(a, b)) = ranges.get(i) else {
            continue;
        };
        shift_down(frags, a, b, dy, ctx.clip);
    }
    acc
}

// First row where cols [col, col+cspan) are all free, extending the table as
// needed.
fn free_row(used: &[Vec<bool>], n: usize, col: usize, cspan: usize) -> usize {
    for (r, row) in used.iter().enumerate() {
        if (col..col + cspan).all(|c| c >= n || !row.get(c).copied().unwrap_or(false)) {
            return r;
        }
    }
    used.len().min(MAX_ROWS - 1)
}

// First free cell in row-major order for an auto-placed item.
fn free_cell(used: &[Vec<bool>], n: usize) -> (usize, usize) {
    for (r, row) in used.iter().enumerate() {
        for c in 0..n {
            if !row.get(c).copied().unwrap_or(false) {
                return (r, c);
            }
        }
    }
    (used.len().min(MAX_ROWS - 1), 0)
}

fn reserve(
    used: &mut Vec<Vec<bool>>,
    n: usize,
    row: usize,
    col: usize,
    cspan: usize,
    rspan: usize,
) {
    let end = (row + rspan).min(MAX_ROWS).max(row + 1).min(MAX_ROWS);
    while used.len() < end {
        let mut r = Vec::new();
        r.resize(n, false);
        used.push(r);
    }
    for r in used.iter_mut().take(end).skip(row) {
        for c in col..(col + cspan).min(n) {
            if let Some(cell) = r.get_mut(c) {
                *cell = true;
            }
        }
    }
}
