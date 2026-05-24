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

use nonos_app_skeleton::PaintBuffer;

use super::button;
use crate::calc::buttons::GRID;
use crate::calc::layout::{cell_origin, cell_size};

pub fn paint(fb: &mut PaintBuffer) {
    let (cw, ch) = cell_size();
    for (row_idx, row) in GRID.iter().enumerate() {
        for (col_idx, btn) in row.iter().enumerate() {
            let (x, y) = cell_origin(row_idx as u32, col_idx as u32, cw, ch);
            button::paint(fb, btn, x, y, cw, ch);
        }
    }
}
