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

use super::blend::blend_rect;
use crate::display::font::get_char_bitmap;
use crate::display::gop::fill_rect;

// A scaled glyph with a two-ring cyan bloom halo, painted over the atmosphere.
pub fn bloom_char(x: u32, y: u32, ch: u8, s: u32, core: u32, glow: u32) {
    let bm = get_char_bitmap(ch);
    halo(x, y, &bm, s, glow, 3, 6, 14);
    halo(x, y, &bm, s, glow, 1, 2, 30);
    for (row, &b) in bm.iter().enumerate() {
        for col in 0..8u32 {
            if (b >> (7 - col)) & 1 == 1 {
                fill_rect(x + col * s, y + (row as u32) * s, s, s, core);
            }
        }
    }
}

fn halo(x: u32, y: u32, bm: &[u8; 16], s: u32, glow: u32, pad: u32, grow: u32, a: u32) {
    for (row, &b) in bm.iter().enumerate() {
        for col in 0..8u32 {
            if (b >> (7 - col)) & 1 == 1 {
                let bx = (x + col * s).saturating_sub(pad);
                let by = (y + (row as u32) * s).saturating_sub(pad);
                blend_rect(bx, by, s + grow, s + grow, glow, a);
            }
        }
    }
}
