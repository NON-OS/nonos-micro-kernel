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

use crate::browser::css::{Computed, GridSpec};

use super::tree::{BoxNode, GridPlace};
use super::walk::Walk;

// Resolve each grid item's requested placement against the container's name
// tables. Items without a request (or anonymous wrappers) stay auto-placed.
pub(super) fn resolve_grid_places(
    w: &Walk,
    container_id: usize,
    style: &Computed,
    kids: &mut [BoxNode],
) {
    let Some(cont) = w.grids.get(container_id).and_then(|s| s.as_ref()) else {
        return;
    };
    let ncols = style.grid_col_n.max(1);
    for kid in kids.iter_mut() {
        if kid.dom_id == 0 {
            continue;
        }
        let Some(item) = w.grids.get(kid.dom_id).and_then(|s| s.as_ref()) else {
            continue;
        };
        if item.places_item() {
            kid.grid_place = place_for(cont, item, ncols);
        }
    }
}

fn place_for(cont: &GridSpec, item: &GridSpec, ncols: u8) -> Option<GridPlace> {
    // grid-area: <name> resolves through the areas table first, then through
    // implicit name-start / name-end column lines.
    if let Some(name) = &item.area {
        if let Some(p) = area_place(cont, name, ncols) {
            return Some(p);
        }
        if let Some(p) = line_pair_place(cont, name, ncols) {
            return Some(p);
        }
        return None;
    }
    let start = item.col_start.as_deref().and_then(|s| line_index(cont, s, ncols));
    let end = item.col_end.as_deref().and_then(|s| line_index(cont, s, ncols));
    let (col, col_span) = match (start, end) {
        (Some(a), Some(b)) if b > a => (a, b - a),
        (Some(a), _) => (a, 1),
        (None, Some(b)) => (b.saturating_sub(1), 1),
        (None, None) if item.row_start.is_none() => return None,
        (None, None) => (0, ncols),
    };
    let row = item.row_start.and_then(row_index);
    let row_span = match (row, item.row_end.and_then(row_index)) {
        (Some(a), Some(b)) if b > a => b - a,
        _ => 1,
    };
    Some(GridPlace {
        col: col.min(ncols - 1),
        col_span: col_span.clamp(1, ncols - col),
        row,
        row_span,
    })
}

// Column and row extent of a named area: the cell range where the name
// appears in the template rows.
fn area_place(cont: &GridSpec, name: &str, ncols: u8) -> Option<GridPlace> {
    let mut c0: Option<u8> = None;
    let mut c1 = 0u8;
    let mut r0: Option<u8> = None;
    let mut r1 = 0u8;
    for (r, row) in cont.areas.iter().enumerate().take(255) {
        for (c, cell) in row.iter().enumerate().take(255) {
            if cell == name {
                let (c, r) = (c as u8, r as u8);
                c0 = Some(c0.map_or(c, |v: u8| v.min(c)));
                c1 = c1.max(c);
                r0 = Some(r0.map_or(r, |v: u8| v.min(r)));
                r1 = r1.max(r);
            }
        }
    }
    let (c0, r0) = (c0?, r0?);
    let col = c0.min(ncols - 1);
    Some(GridPlace {
        col,
        col_span: (c1 - c0 + 1).clamp(1, ncols - col),
        row: Some(r0),
        row_span: r1 - r0 + 1,
    })
}

// Implicit area from named lines: name-start opens the span, name-end closes.
fn line_pair_place(cont: &GridSpec, name: &str, ncols: u8) -> Option<GridPlace> {
    let mut start: Option<u8> = None;
    let mut end: Option<u8> = None;
    for (n, idx) in &cont.col_lines {
        if let Some(base) = n.strip_suffix("-start") {
            if base == name {
                start = Some(*idx);
            }
        }
        if let Some(base) = n.strip_suffix("-end") {
            if base == name {
                end = Some(*idx);
            }
        }
    }
    let col = start?.min(ncols - 1);
    let span = end.filter(|e| *e > col).map_or(1, |e| e - col);
    Some(GridPlace { col, col_span: span.clamp(1, ncols - col), row: None, row_span: 1 })
}

// One grid-column line: a 1-based number (negative counts from the end) or a
// named line. Returns the zero-based track index the line opens.
fn line_index(cont: &GridSpec, token: &str, ncols: u8) -> Option<u8> {
    let t = token.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("auto") || t.starts_with("span") {
        return None;
    }
    if let Ok(n) = t.parse::<i16>() {
        let lines = ncols as i16 + 1;
        let line = if n < 0 { lines + n + 1 } else { n };
        if line < 1 {
            return None;
        }
        return Some(((line - 1) as u8).min(ncols - 1));
    }
    cont.col_lines.iter().find(|(n, _)| n.as_str() == t).map(|(_, i)| (*i).min(ncols - 1))
}

// Rows are 1-based in CSS; the placement grid is zero-based.
fn row_index(n: i16) -> Option<u8> {
    if (1..=255).contains(&n) {
        Some((n - 1) as u8)
    } else {
        None
    }
}
