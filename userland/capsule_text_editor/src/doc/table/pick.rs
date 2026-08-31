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

//! Turn a click inside a table row back into a byte offset. The column comes
//! from the same width list the painter laid the cells out with, then the
//! nearest character boundary within that cell wins.

use crate::doc::document::Doc;
use crate::doc::linebox::LineBox;
use crate::doc::measure::Measurer;
use crate::doc::table::geom::{col_at, col_x, CELL_PAD_X};
use crate::doc::table::run::run_of;
use crate::doc::table::syntax::{cell_span, is_row};
use crate::doc::table::widths::col_widths;

pub fn offset_at_row(
    doc: &Doc,
    line: &LineBox,
    x: f32,
    content_w: f32,
    m: &dyn Measurer,
) -> Option<usize> {
    let b = doc.blocks.get(line.block)?;
    let t = b.as_str();
    if !is_row(t) {
        return None;
    }
    let w = col_widths(doc, run_of(doc, line.block)?, content_w, m);
    if w.is_empty() {
        return None;
    }
    let col = col_at(&w, x);
    let (s, e) = cell_span(t, col)?;
    let cell = t.get(s..e)?;
    let st = b.style_at(0);
    let ox = col_x(&w, col) + CELL_PAD_X;
    let mut best = 0usize;
    let mut best_d = f32::MAX;
    for (i, _) in cell.char_indices().chain(core::iter::once((cell.len(), ' '))) {
        let d = (ox + m.advance(&cell[..i], &st) - x).abs();
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    Some(s + best)
}
