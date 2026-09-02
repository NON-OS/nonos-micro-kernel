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

use super::types::Scrollback;

impl Scrollback {
    /// Append a listing row whose trailing name is a directory, so the name
    /// alone takes the directory colour and the columns before it do not.
    ///
    /// While a capture is active the two halves are joined and redirected
    /// through push_line, exactly as push_error does: a pipe downstream of
    /// `ls` reads what the user typed, never the colour metadata.
    pub fn push_dir_row(&mut self, plain: &[u8], name: &[u8]) {
        if self.capture.is_some() {
            let mut joined = alloc::vec::Vec::from(plain);
            joined.extend_from_slice(name);
            self.push_line(&joined);
            return;
        }
        self.grid.feed(plain);
        self.grid.feed(b"\x1b[94m");
        self.grid.feed(name);
        self.grid.feed(b"\x1b[0m\n");
    }
}
