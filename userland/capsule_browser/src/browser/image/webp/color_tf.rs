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

// The color transform decorrelates red and blue from green per block. The
// stored element packs three signed multipliers; the delta is (t * c) >> 5
// with both operands taken as signed bytes.
fn delta(t: u32, c: i32) -> i32 {
    ((t as u8 as i8 as i32) * c) >> 5
}

// Undo the color transform in place, one block element per pixel block.
pub(super) fn apply(px: &mut [u32], w: usize, h: usize, data: &[u32], bits: u32, dw: usize) {
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let m = data[(y >> bits) * dw + (x >> bits)];
            let g2r = m & 0xff;
            let g2b = (m >> 8) & 0xff;
            let r2b = (m >> 16) & 0xff;
            let argb = px[i];
            let green = ((argb >> 8) & 0xff) as u8 as i8 as i32;
            let mut red = (argb >> 16) & 0xff;
            red = red.wrapping_add(delta(g2r, green) as u32) & 0xff;
            let mut blue = argb & 0xff;
            blue = blue.wrapping_add(delta(g2b, green) as u32) & 0xff;
            blue = blue.wrapping_add(delta(r2b, red as u8 as i8 as i32) as u32) & 0xff;
            px[i] = (argb & 0xff00_ff00) | (red << 16) | blue;
        }
    }
}
