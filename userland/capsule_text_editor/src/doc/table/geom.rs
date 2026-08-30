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

//! The one place a table's geometry is computed. The painter and the caret
//! hit-test both come through here, so a click lands on the cell that was
//! drawn under it instead of drifting from it.

use crate::doc::document::Doc;
use crate::doc::measure::Measurer;
use crate::doc::style::RunStyle;
use crate::doc::table::syntax::col_count;

pub const CELL_PAD_X: f32 = 8.0;
pub const ROW_PAD_Y: f32 = 5.0;
pub const MIN_COL_W: f32 = 24.0;

pub fn row_height(style: &RunStyle, m: &dyn Measurer) -> f32 {
    m.line_height(style) + 2.0 * ROW_PAD_Y
}

pub fn col_count_of(doc: &Doc, run: (usize, usize)) -> usize {
    (run.0..run.1)
        .filter_map(|i| doc.blocks.get(i))
        .map(|b| col_count(b.as_str()))
        .max()
        .unwrap_or(0)
}

pub fn col_x(widths: &[f32], col: usize) -> f32 {
    widths.iter().take(col).sum()
}

pub fn col_at(widths: &[f32], x: f32) -> usize {
    let mut acc = 0.0f32;
    for (i, w) in widths.iter().enumerate() {
        acc += *w;
        if x < acc {
            return i;
        }
    }
    widths.len().saturating_sub(1)
}
