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

//! Cutting the status line to the room it actually has.
//!
//! The bar draws a message from the left and the chain name from the right,
//! and nothing stopped the two meeting. Short messages hid it; a sentence
//! explaining why a stored wallet would not open is long enough to run under
//! the chain name and read as one corrupted line.
//!
//! Measured, not counted: the face is proportional, so a character budget is
//! wrong by a different amount for every message.

use nonos_app_skeleton::PaintBuffer;

use super::scale;

/// The longest prefix of `msg` that fits `room`, with an ellipsis when it had
/// to cut. Cuts on a character boundary, so a multi-byte glyph is never split
/// into bytes that are not text.
pub fn fit<'a>(fb: &PaintBuffer, msg: &'a str, room: u32) -> &'a str {
    if fb.measure_ttf(msg, scale::BODY).max(0) as u32 <= room {
        return msg;
    }
    let mut end = msg.len();
    while end > 0 {
        if msg.is_char_boundary(end) {
            let cut = &msg[..end];
            if fb.measure_ttf(cut, scale::BODY).max(0) as u32 <= room {
                return cut;
            }
        }
        end -= 1;
    }
    ""
}
