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

use super::state::State;

impl State {
    /// Delete the character immediately before the caret (undoable).
    pub fn backspace(&mut self) -> bool {
        if self.caret == 0 {
            return false;
        }
        let mut start = self.caret - 1;
        while start > 0 && self.buf[start] & 0b1100_0000 == 0b1000_0000 {
            start -= 1;
        }
        self.apply_edit(start, self.caret - start, &[])
    }
}
