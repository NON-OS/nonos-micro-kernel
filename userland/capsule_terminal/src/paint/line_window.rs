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

//! Which slice of a long line is on screen.

use super::line_chars::char_floor;

/// The byte range of the line to draw, so the cursor is always visible.
///
/// Measured in cells but indexing bytes, so both ends are moved back to a
/// character boundary: cutting one in half would draw the rest of the line
/// as damage.
pub fn window(body: &[u8], cursor: usize, cells: usize) -> (usize, usize, usize) {
    let scroll = if cursor < cells { 0 } else { cursor - cells + 1 };
    let end = (scroll + cells).min(body.len());
    let start = char_floor(body, scroll);
    let stop = char_floor(body, end).max(start);
    (start, stop, scroll)
}
