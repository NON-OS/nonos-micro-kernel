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

pub(super) fn end_position(bytes: &[u8], wrap_cols: u32) -> (u32, u32) {
    let mut line = 0;
    let mut col = 0;
    let text = core::str::from_utf8(bytes).unwrap_or("");
    for ch in text.chars() {
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            if col == wrap_cols {
                line += 1;
                col = 0;
            }
            col += 1;
        }
    }
    if col == wrap_cols {
        (line + 1, 0)
    } else {
        (line, col)
    }
}
