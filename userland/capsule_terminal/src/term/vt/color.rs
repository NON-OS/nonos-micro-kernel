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

// Colours are stored per cell as full ARGB now, so the whole 24-bit space from
// `38;2;r;g;b` survives instead of being rounded to sixteen. The default
// background is fully transparent so an unstyled cell shows the terminal's own
// backdrop rather than a solid black.
pub const DEFAULT_FG: u32 = 0xFFC0_C0C0;
pub const DEFAULT_BG: u32 = 0x0000_0000;

// Pack a 24-bit colour into opaque ARGB.
pub fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn ansi_to_argb(index: u8) -> u32 {
    let pack = |r: u8, g: u8, b: u8| -> u32 {
        0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    };
    match index {
        0 => pack(0x00, 0x00, 0x00),
        1 => pack(0x80, 0x00, 0x00),
        2 => pack(0x00, 0x80, 0x00),
        3 => pack(0x80, 0x80, 0x00),
        4 => pack(0x00, 0x00, 0x80),
        5 => pack(0x80, 0x00, 0x80),
        6 => pack(0x00, 0x80, 0x80),
        7 => pack(0xc0, 0xc0, 0xc0),
        8 => pack(0x80, 0x80, 0x80),
        9 => pack(0xff, 0x00, 0x00),
        10 => pack(0x00, 0xff, 0x00),
        11 => pack(0xff, 0xff, 0x00),
        12 => pack(0x00, 0x00, 0xff),
        13 => pack(0xff, 0x00, 0xff),
        14 => pack(0x00, 0xff, 0xff),
        15 => pack(0xff, 0xff, 0xff),
        16..=231 => {
            let convert = |v: u8| if v == 0 { 0u8 } else { v * 0x28 + 0x28 };
            let r = convert((index - 16) / 36 % 6);
            let g = convert((index - 16) / 6 % 6);
            let b = convert((index - 16) % 6);
            pack(r, g, b)
        }
        232..=255 => {
            let gray = (index - 232) * 10 + 8;
            pack(gray, gray, gray)
        }
    }
}
