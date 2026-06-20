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

use crate::term::dimensions::{COLS, SCROLLBACK_ROWS, VISIBLE_ROWS};
use crate::term::grid::cell::Cell;
use crate::term::grid::types::Grid;

impl Grid {
    pub fn visible_row(&self, i: usize) -> &[Cell] {
        let ci = (self.hist_count + i).saturating_sub(self.view_offset);
        if ci < self.hist_count {
            let slot = (self.hist_head + ci) % SCROLLBACK_ROWS;
            &self.history[slot * COLS..slot * COLS + COLS]
        } else {
            let live = (ci - self.hist_count).min(VISIBLE_ROWS - 1);
            &self.cells[live * COLS..live * COLS + COLS]
        }
    }
}
