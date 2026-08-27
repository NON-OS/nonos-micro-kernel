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

use super::metrics::PAD_TIGHT;
use super::rank_geom::table;
use super::rect::{self, Rect};

pub const COLUMNS: usize = 4;

pub const HEADS: [&[u8]; COLUMNS] = [b"#", b"Score", b"Mode", b"Length"];

// Weights, not pixels: the table is as wide as the window leaves it, and a
// fixed column would clip the mode name at the first resize.
const WEIGHT: [u32; COLUMNS] = [1, 3, 3, 2];

fn total() -> u32 {
    let mut sum = 0;
    for weight in WEIGHT {
        sum += weight;
    }
    sum.max(1)
}

pub fn column(w: u32, h: u32, index: usize) -> (u32, u32) {
    let inner = rect::inset(table(w, h), PAD_TIGHT);
    let index = index.min(COLUMNS - 1);
    let denom = total();
    let prior: u32 = (0..index).map(|i| inner.2 * WEIGHT[i] / denom).sum();
    (inner.0 + prior, inner.2 * WEIGHT[index] / denom)
}

pub fn cell(band: Rect, w: u32, h: u32, index: usize) -> Rect {
    let (x, width) = column(w, h, index);
    (x, band.1, width, band.3)
}
