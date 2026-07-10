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

use crate::term::dimensions::{COLS, VISIBLE_ROWS};
use crate::term::grid::types::Grid;

impl Grid {
    pub fn move_cells(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
        w: usize,
        h: usize,
    ) {
        for ry in 0..h {
            let fy = from_y + ry;
            let ty = to_y + ry;
            if fy >= VISIBLE_ROWS || ty >= VISIBLE_ROWS {
                continue;
            }
            let fw = w.min(COLS.saturating_sub(from_x));
            let tw = fw.min(COLS.saturating_sub(to_x));
            if tw == 0 {
                continue;
            }
            let src = Grid::idx(from_x, fy);
            let dst = Grid::idx(to_x, ty);
            self.cells.copy_within(src..src + tw, dst);
        }
    }
}
