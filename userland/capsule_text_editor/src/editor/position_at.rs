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

//! Visual (line, column) of a byte offset, using the exact same wrap rule the
//! painter draws with, so the caret always sits where the character it points
//! at is rendered.

pub(super) fn position_at(bytes: &[u8], wrap: u32, target: usize) -> (u32, u32) {
    let text = core::str::from_utf8(bytes).unwrap_or("");
    let mut line = 0;
    let mut col = 0;
    for (bi, ch) in text.char_indices() {
        if bi >= target {
            // The caret sits just before this character; a pending wrap moves it
            // to the start of the next visual line.
            if ch != '\n' && col == wrap {
                return (line + 1, 0);
            }
            return (line, col);
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            if col == wrap {
                line += 1;
                col = 0;
            }
            col += 1;
        }
    }
    if col == wrap {
        (line + 1, 0)
    } else {
        (line, col)
    }
}
