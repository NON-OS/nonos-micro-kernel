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

//! Where the caret sits inside a table row. The offset is a plain byte offset
//! into the row's own text, so the column it belongs to and the pixels it
//! belongs at both fall out of the same geometry the painter used.

use crate::doc::document::Doc;
use crate::doc::linebox::LineBox;
use crate::doc::measure::Measurer;
use crate::doc::table::geom::{col_x, CELL_PAD_X, ROW_PAD_Y};
use crate::doc::table::run::run_of;
use crate::doc::table::syntax::{cell_span, col_of_offset, is_row};
use crate::doc::table::widths::col_widths;

pub fn caret_in_row(
    doc: &Doc,
    line: &LineBox,
    off: usize,
    content_w: f32,
    m: &dyn Measurer,
) -> Option<(f32, f32, f32)> {
    let b = doc.blocks.get(line.block)?;
    let t = b.as_str();
    if !is_row(t) {
        return None;
    }
    let w = col_widths(doc, run_of(doc, line.block)?, content_w, m);
    let col = col_of_offset(t, off);
    let (s, e) = cell_span(t, col)?;
    let cut = off.clamp(s, e);
    let st = b.style_at(0);
    let x = col_x(&w, col) + CELL_PAD_X + m.advance(t.get(s..cut).unwrap_or(""), &st);
    Some((x, line.y + ROW_PAD_Y, m.line_height(&st)))
}
