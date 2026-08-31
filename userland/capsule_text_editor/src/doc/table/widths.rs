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

//! Column widths, measured from the real glyph advances of every cell in the
//! run and then scaled as one so the grid always fills the content column
//! exactly, whether the natural widths overflow it or fall short.

use alloc::vec;
use alloc::vec::Vec;

use crate::doc::document::Doc;
use crate::doc::measure::Measurer;
use crate::doc::table::geom::{col_count_of, CELL_PAD_X, MIN_COL_W};
use crate::doc::table::syntax::cell_span;

pub fn col_widths(doc: &Doc, run: (usize, usize), content_w: f32, m: &dyn Measurer) -> Vec<f32> {
    let cols = col_count_of(doc, run);
    if cols == 0 {
        return Vec::new();
    }
    let mut w = vec![MIN_COL_W; cols];
    for b in (run.0..run.1).filter_map(|i| doc.blocks.get(i)) {
        let (t, st) = (b.as_str(), b.style_at(0));
        for (c, slot) in w.iter_mut().enumerate() {
            let Some((s, e)) = cell_span(t, c) else { continue };
            let need = m.advance(&t[s..e], &st) + 2.0 * CELL_PAD_X;
            if need > *slot {
                *slot = need;
            }
        }
    }
    let sum: f32 = w.iter().sum();
    if sum > 0.0 && content_w > 0.0 {
        let k = content_w / sum;
        for slot in w.iter_mut() {
            *slot *= k;
        }
    }
    w
}
