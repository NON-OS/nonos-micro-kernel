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

impl Grid {
    pub fn current_abs_line(&self) -> u64 {
        self.total_scrolled + self.y as u64
    }

    pub fn abs_base(&self) -> u64 {
        self.total_scrolled - self.hist_count as u64
    }

    pub fn abs_of_visible_row(&self, row: usize) -> u64 {
        self.total_scrolled + row as u64 - self.view_offset as u64
    }
}
