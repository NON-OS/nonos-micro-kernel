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
    /// DECSTBM: set the vertical scrolling region to the inclusive 0-based rows
    /// `[top, bot]`. An inverted or out-of-range window resets to the full
    /// screen, matching xterm. Per the spec the cursor homes to the top-left of
    /// the new region afterwards.
    pub fn set_scroll_region(&mut self, top: usize, bot: usize) {
        if top < bot && bot < VISIBLE_ROWS {
            self.scroll_top = top;
            self.scroll_bot = bot;
        } else {
            self.scroll_top = 0;
            self.scroll_bot = VISIBLE_ROWS - 1;
        }
        self.x = 0;
        self.y = self.scroll_top;
    }

    /// Scroll the active region up by one line. A full-screen region preserves
    /// its top line into scrollback (the normal terminal history path); a
    /// partial region simply drops its top line, so a pinned status bar outside
    /// the window is left untouched.
    pub fn scroll_region_up(&mut self) {
        if self.scroll_top == 0 && self.scroll_bot == VISIBLE_ROWS - 1 {
            self.scroll_up_one();
            return;
        }
        let (top, bot) = (self.scroll_top, self.scroll_bot);
        for y in top..bot {
            let dst = y * COLS;
            let src = (y + 1) * COLS;
            self.cells.copy_within(src..src + COLS, dst);
        }
        let blank = self.blank_cell();
        for x in 0..COLS {
            self.cells[Grid::idx(x, bot)] = blank;
        }
    }
}
