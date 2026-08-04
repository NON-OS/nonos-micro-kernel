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

//! Drawing a line of text into cells.

use nonos_app_skeleton::PaintBuffer;

use crate::term::grid::width::char_width;

// Draw text as crisp monospace, advancing by the columns each character
// occupies. What is typed has to render the same as what the grid shows, or a
// line looks different while it is being written than after it has been run.
pub fn text(fb: &mut PaintBuffer, mut x: u32, y: u32, bytes: &[u8], argb: u32, adv: u32, px: f32) {
    let mut buf = [0u8; 4];
    for ch in chars_of(bytes) {
        if ch != ' ' && (ch as u32) >= 0x20 && ch as u32 != 0x7f {
            let s = ch.encode_utf8(&mut buf);
            let _ = fb.text_ttf_mono(x as i32, y as i32, s, argb, px);
        }
        x += adv * char_width(ch) as u32;
    }
}

// The line is held as bytes and the scroll window can cut it mid character,
// so only the part that is whole is drawn. The tail is at most one partial
// character and arrives complete on the next keystroke.
fn chars_of(bytes: &[u8]) -> impl Iterator<Item = char> + '_ {
    let whole = match core::str::from_utf8(bytes) {
        Ok(text) => text,
        Err(err) => core::str::from_utf8(&bytes[..err.valid_up_to()]).unwrap_or(""),
    };
    whole.chars()
}

// The nearest character boundary at or before `at`. A byte in the middle of a
// character has its top two bits set to one and zero, which is what marks it
// as a continuation of the byte before.
pub fn char_floor(bytes: &[u8], at: usize) -> usize {
    let mut i = at.min(bytes.len());
    while i > 0 && bytes.get(i).is_some_and(|b| b & 0xC0 == 0x80) {
        i -= 1;
    }
    i
}
