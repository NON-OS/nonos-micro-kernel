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
use crate::display::gop::put_pixel;

pub fn draw_char(x: u32, y: u32, ch: u8, c: u32) {
    for (row, &b) in get_char_bitmap(ch).iter().enumerate() {
        for col in 0..8u32 {
            if (b >> (7 - col)) & 1 == 1 {
                put_pixel(x + col, y + row as u32, c);
            }
        }
    }
}

pub fn draw_string(x: u32, y: u32, s: &[u8], c: u32) {
    let mut cx = x;
    for &ch in s {
        if ch != b'\n' {
            draw_char(cx, y, ch, c);
            cx += CHAR_WIDTH;
        }
    }
}

pub fn draw_char_2x(x: u32, y: u32, ch: u8, c: u32) {
    for (row, &b) in get_char_bitmap(ch).iter().enumerate() {
        for col in 0..8u32 {
            if (b >> (7 - col)) & 1 == 1 {
                let (px, py) = (x + col * 2, y + (row as u32) * 2);
                put_pixel(px, py, c);
                put_pixel(px + 1, py, c);
                put_pixel(px, py + 1, c);
                put_pixel(px + 1, py + 1, c);
            }
        }
    }
}

pub fn draw_string_2x(x: u32, y: u32, s: &[u8], c: u32) {
    let mut cx = x;
    for &ch in s {
        if ch != b'\n' {
            draw_char_2x(cx, y, ch, c);
            cx += CHAR_WIDTH * 2;
        }
    }
}

pub fn draw_hex_byte(x: u32, y: u32, b: u8, c: u32) {
    const H: &[u8] = b"0123456789ABCDEF";
    draw_char(x, y, H[(b >> 4) as usize], c);
    draw_char(x + CHAR_WIDTH, y, H[(b & 0x0F) as usize], c);
}

pub fn draw_hash_bytes(x: u32, y: u32, hash: &[u8], r: usize, c: u32, d: u32) {
    for (i, &b) in hash.iter().enumerate() {
        draw_hex_byte(
            x + (i as u32 * CHAR_WIDTH * 2),
            y,
            if i < r { b } else { 0 },
            if i < r { c } else { d },
        );
    }
}
