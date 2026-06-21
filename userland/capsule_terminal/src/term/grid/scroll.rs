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
use crate::term::dimensions::{COLS, VISIBLE_ROWS, SCROLLBACK_ROWS};

impl Grid {
    pub fn scroll_up_one(&mut self) {
        let slot = (self.hist_head + self.hist_count) % SCROLLBACK_ROWS;
        let dst = slot * COLS;
        for x in 0..COLS {
            self.history[dst + x] = self.cells[Grid::idx(x, 0)];
        }
        if self.hist_count < SCROLLBACK_ROWS {
            self.hist_count += 1;
        } else {
            self.hist_head = (self.hist_head + 1) % SCROLLBACK_ROWS;
        }
        self.total_scrolled += 1;
        self.cells.copy_within(COLS..VISIBLE_ROWS * COLS, 0);
        let blank = self.blank_cell();
        for x in 0..COLS {
            self.cells[Grid::idx(x, VISIBLE_ROWS - 1)] = blank;
        }
    }
}
