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
use crate::term::grid::cell::{Cell, F_WIDE_TAIL};
use crate::term::grid::types::Grid;
use crate::term::grid::width::char_width;

impl Grid {
    pub fn blank_cell(&self) -> Cell {
        Cell { ch: ' ', fg: self.fg, bg: self.bg, flags: 0 }
    }
    pub fn put_char(&mut self, c: char) {
        let w = char_width(c);
        // A character drawn two columns wide cannot straddle the edge, so it
        // wraps whole rather than being split across two lines.
        if self.x + w > COLS {
            self.x = 0;
            self.line_feed();
        }
        let i = Grid::idx(self.x, self.y);
        self.cells[i] = Cell { ch: c, fg: self.fg, bg: self.bg, flags: self.flags };
        if w == 2 {
            // The right half holds no glyph. It is still a cell, so that
            // erasing, scrolling and background fills treat the pair as the
            // two columns it occupies.
            let tail = Grid::idx(self.x + 1, self.y);
            self.cells[tail] =
                Cell { ch: ' ', fg: self.fg, bg: self.bg, flags: self.flags | F_WIDE_TAIL };
        }
        self.x += w;
        if self.x >= COLS {
            self.x = 0;
            self.line_feed();
        }
    }
    pub fn line_feed(&mut self) {
        if self.y == self.scroll_bot {
            // At the foot of the scroll region the window shifts and the cursor
            // stays put; elsewhere it just steps down within the screen.
            self.scroll_region_up();
        } else if self.y + 1 < VISIBLE_ROWS {
            self.y += 1;
        }
    }
    pub fn carriage_return(&mut self) {
        self.x = 0;
    }
}
