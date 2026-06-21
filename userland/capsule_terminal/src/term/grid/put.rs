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

use crate::term::grid::cell::Cell;
use crate::term::grid::types::Grid;
use crate::term::dimensions::{COLS, VISIBLE_ROWS};

impl Grid {
    pub fn blank_cell(&self) -> Cell {
        Cell { ch: b' ', fg: self.fg, bg: self.bg, flags: 0 }
    }
    pub fn put_char(&mut self, c: u8) {
        let i = Grid::idx(self.x, self.y);
        self.cells[i] = Cell { ch: c, fg: self.fg, bg: self.bg, flags: self.flags };
        self.x += 1;
        if self.x >= COLS {
            self.x = 0;
            self.line_feed();
        }
    }
    pub fn line_feed(&mut self) {
        self.y += 1;
        if self.y >= VISIBLE_ROWS {
            self.scroll_up_one();
            self.y = VISIBLE_ROWS - 1;
        }
    }
    pub fn carriage_return(&mut self) {
        self.x = 0;
    }
}
