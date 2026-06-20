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

use crate::term::grid::types::Grid;
use crate::term::dimensions::{COLS, VISIBLE_ROWS};

impl Grid {
    pub fn erase_line(&mut self, mode: u8) {
        let blank = self.blank_cell();
        let (start, end) = match mode {
            0 => (self.x, COLS),
            1 => (0, self.x),
            _ => (0, COLS),
        };
        for x in start..end {
            self.cells[Grid::idx(x, self.y)] = blank;
        }
    }
    pub fn erase_display(&mut self, mode: u8) {
        let blank = self.blank_cell();
        match mode {
            0 => {
                for x in self.x..COLS { self.cells[Grid::idx(x, self.y)] = blank; }
                for y in (self.y + 1)..VISIBLE_ROWS {
                    for x in 0..COLS { self.cells[Grid::idx(x, y)] = blank; }
                }
            }
            1 => {
                for y in 0..self.y {
                    for x in 0..COLS { self.cells[Grid::idx(x, y)] = blank; }
                }
                for x in 0..self.x { self.cells[Grid::idx(x, self.y)] = blank; }
            }
            _ => { self.clear(); }
        }
    }
    pub fn clear(&mut self) {
        let blank = self.blank_cell();
        for i in 0..self.cells.len() { self.cells[i] = blank; }
        self.x = 0;
        self.y = 0;
        self.view_offset = 0;
    }
}
