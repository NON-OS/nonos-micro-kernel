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

use super::types::Line;

impl Line {
    // Delete the word before the cursor (Ctrl-W): skip any spaces, then
    // remove the run of non-space characters, closing the gap.
    pub fn delete_word(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        let mut start = self.cursor;
        while start > 0 && self.buf[start - 1] == b' ' {
            start -= 1;
        }
        while start > 0 && self.buf[start - 1] != b' ' {
            start -= 1;
        }
        let removed = self.cursor - start;
        if removed == 0 {
            return false;
        }
        if self.cursor < self.len {
            self.buf.copy_within(self.cursor..self.len, start);
        }
        self.len -= removed;
        self.cursor = start;
        true
    }
}
