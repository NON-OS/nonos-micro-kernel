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
    pub fn scroll_view_up(&mut self, lines: usize) {
        self.view_offset = (self.view_offset + lines).min(self.hist_count);
    }
    pub fn scroll_view_down(&mut self, lines: usize) {
        self.view_offset = self.view_offset.saturating_sub(lines);
    }
    pub fn jump_view_bottom(&mut self) {
        self.view_offset = 0;
    }
}
