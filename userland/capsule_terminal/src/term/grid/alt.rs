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

use crate::term::dimensions::VISIBLE_ROWS;
use crate::term::grid::types::Grid;

impl Grid {
    pub fn enter_alt(&mut self, clear: bool) {
        if !self.alternate {
            core::mem::swap(&mut self.cells, &mut self.alt);
            self.alternate = true;
        }
        // A fresh screen starts with a full-screen scroll region.
        self.scroll_top = 0;
        self.scroll_bot = VISIBLE_ROWS - 1;
        if clear {
            self.clear();
        }
    }

    pub fn leave_alt(&mut self) {
        if self.alternate {
            core::mem::swap(&mut self.cells, &mut self.alt);
            self.alternate = false;
        }
        self.scroll_top = 0;
        self.scroll_bot = VISIBLE_ROWS - 1;
    }
}
