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

use super::line_chars::chars_of;
use super::syntax::Part;
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

// Draw a line with each part in its own colour. Same advance rule as `text`,
// so a highlighted line and a plain one occupy the same columns.
pub fn text_parts(
    fb: &mut PaintBuffer,
    mut x: u32,
    y: u32,
    bytes: &[u8],
    parts: &[Part],
    adv: u32,
    px: f32,
) {
    let mut buf = [0u8; 4];
    let mut at = 0usize;
    for ch in chars_of(bytes) {
        if ch != ' ' && (ch as u32) >= 0x20 && ch as u32 != 0x7f {
            let argb = parts.get(at).copied().unwrap_or(Part::Plain).colour();
            let s = ch.encode_utf8(&mut buf);
            let _ = fb.text_ttf_mono(x as i32, y as i32, s, argb, px);
        }
        x += adv * char_width(ch) as u32;
        at += ch.len_utf8();
    }
}
