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
use crate::font::glyph::GlyphBitmap;

pub fn draw_glyph(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    glyph: &GlyphBitmap,
    color: u32,
) {
    let w = w as usize;
    let h = h as usize;
    for row in 0..glyph.height as usize {
        for col in 0..glyph.width as usize {
            if glyph.rows[row] & (0x80 >> col) == 0 {
                continue;
            }
            let px = x as usize + col;
            let py = y as usize + row;
            if px >= w || py >= h {
                continue;
            }
            let idx = py.saturating_mul(stride).saturating_add(px);
            if idx < buf.len() {
                buf[idx] = color;
            }
        }
    }
}
