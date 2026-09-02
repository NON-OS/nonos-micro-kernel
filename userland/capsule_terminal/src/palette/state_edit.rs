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

use super::state::{Palette, QUERY_CAP};

impl Palette {
    pub fn push(&mut self, b: u8) {
        if self.qlen < QUERY_CAP {
            self.query[self.qlen] = b;
            self.qlen += 1;
        }
        self.sel = 0;
    }

    pub fn backspace(&mut self) {
        self.qlen = self.qlen.saturating_sub(1);
        self.sel = 0;
    }

    /// Wraps, so holding Down walks the list instead of sticking at the end.
    pub fn step(&mut self, delta: i32, count: usize) {
        if count == 0 {
            self.sel = 0;
            return;
        }
        let n = count as i32;
        self.sel = (((self.sel as i32 + delta) % n + n) % n) as usize;
    }
}
