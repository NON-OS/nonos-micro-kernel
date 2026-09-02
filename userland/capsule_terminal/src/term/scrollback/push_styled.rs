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
    /// Append a line that carries its own colour escapes on screen and none
    /// at all through a pipe.
    ///
    /// The two forms are supplied by the caller rather than derived, because
    /// stripping escapes after the fact cannot tell a colour code from text
    /// the user typed. While a capture is active only `plain` is emitted, so a
    /// file downstream never receives colour metadata.
    pub fn push_styled(&mut self, plain: &[u8], styled: &[u8]) {
        if self.capture.is_some() {
            self.push_line(plain);
            return;
        }
        self.grid.feed(styled);
        self.grid.feed(b"\x1b[0m\n");
    }
}
