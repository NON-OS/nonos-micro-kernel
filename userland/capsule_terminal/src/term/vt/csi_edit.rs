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

use crate::term::dimensions::COLS;
use crate::term::grid::types::Grid;

fn n1(params: &[i64]) -> usize {
    params.first().copied().unwrap_or(1).max(1) as usize
}

pub fn csi_edit(g: &mut Grid, c: u8, params: &[i64]) {
    match c {
        b'J' => {
            g.erase_display(params.first().copied().unwrap_or(0) as u8);
        }
        b'K' => {
            g.erase_line(params.first().copied().unwrap_or(0) as u8);
        }
        b'P' => {
            let n = n1(params).min(COLS - g.x);
            g.move_cells(g.x + n, g.y, g.x, g.y, COLS - (g.x + n), 1);
            let bc = g.blank_cell();
            for x in (COLS - n)..COLS {
                let i = Grid::idx(x, g.y);
                g.cells[i] = bc;
            }
        }
        b'@' => {
            let n = n1(params).min(COLS - g.x);
            g.move_cells(g.x, g.y, g.x + n, g.y, COLS - (g.x + n), 1);
            let bc = g.blank_cell();
            for x in g.x..(g.x + n) {
                let i = Grid::idx(x, g.y);
                g.cells[i] = bc;
            }
        }
        _ => {}
    }
}
