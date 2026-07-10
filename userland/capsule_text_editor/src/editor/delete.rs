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

//! Forward delete: remove the character at the caret, leaving the caret put.

use super::state::State;

impl State {
    pub fn delete_forward(&mut self) -> bool {
        if self.caret >= self.len {
            return false;
        }
        let mut end = self.caret + 1;
        while end < self.len && self.buf[end] & 0b1100_0000 == 0b1000_0000 {
            end += 1;
        }
        self.apply_edit(self.caret, end - self.caret, &[])
    }
}
