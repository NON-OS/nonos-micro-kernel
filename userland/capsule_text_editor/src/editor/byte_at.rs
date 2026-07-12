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

//! Inverse of position_at: the byte offset nearest a visual (line, column).
//! Used for click-to-place and for keeping the column when moving up or down.
//! A column past the end of a line clamps to that line's end; a line past the
//! end clamps to the end of the buffer.

pub(super) fn byte_at(bytes: &[u8], wrap: u32, tline: u32, tcol: u32) -> usize {
    let text = core::str::from_utf8(bytes).unwrap_or("");
    let mut line = 0;
    let mut col = 0;
    let mut line_end = None;
    for (bi, ch) in text.char_indices() {
        // Where the caret slot before this character renders.
        let (sl, sc) = if ch != '\n' && col == wrap { (line + 1, 0) } else { (line, col) };
        if sl == tline {
            if sc == tcol {
                return bi;
            }
            line_end = Some(bi);
        } else if sl > tline {
            // Walked past the target line; land on its end if we saw it.
            return line_end.unwrap_or(bi);
        }
        if ch == '\n' {
            line = sl + 1;
            col = 0;
        } else {
            line = sl;
            col = sc + 1;
        }
    }
    // Reached the end of the buffer without an exact hit: the target is at or
    // beyond the final caret slot, so clamp to the end.
    let _ = line_end;
    bytes.len()
}
