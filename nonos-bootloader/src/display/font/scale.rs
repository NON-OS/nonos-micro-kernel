// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::bitmap::get_char_bitmap;
use super::CHAR_WIDTH;
use crate::display::gop::fill_rect;

pub fn draw_char_3x(x: u32, y: u32, ch: u8, c: u32) {
    for (row, &b) in get_char_bitmap(ch).iter().enumerate() {
        for col in 0..8u32 {
            if (b >> (7 - col)) & 1 == 1 {
                fill_rect(x + col * 3, y + (row as u32) * 3, 3, 3, c);
            }
        }
    }
}

pub fn draw_string_3x(x: u32, y: u32, s: &[u8], c: u32) {
    let mut cx = x;
    for &ch in s {
        if ch != b'\n' {
            draw_char_3x(cx, y, ch, c);
            cx += CHAR_WIDTH * 3;
        }
    }
}

pub fn draw_char_4x(x: u32, y: u32, ch: u8, c: u32) {
    for (row, &b) in get_char_bitmap(ch).iter().enumerate() {
        for col in 0..8u32 {
            if (b >> (7 - col)) & 1 == 1 {
                fill_rect(x + col * 4, y + (row as u32) * 4, 4, 4, c);
            }
        }
    }
}

pub fn draw_string_4x(x: u32, y: u32, s: &[u8], c: u32) {
    let mut cx = x;
    for &ch in s {
        if ch != b'\n' {
            draw_char_4x(cx, y, ch, c);
            cx += CHAR_WIDTH * 4;
        }
    }
}
