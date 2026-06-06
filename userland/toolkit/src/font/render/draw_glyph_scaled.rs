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

pub fn draw_glyph_scaled(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    glyph: &GlyphBitmap,
    color: u32,
    scale: u32,
) {
    for row in 0..glyph.height as u32 {
        for col in 0..glyph.width as u32 {
            if glyph.rows[row as usize] & (0x80 >> col) == 0 {
                continue;
            }
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = x + col * scale + dx;
                    let py = y + row * scale + dy;
                    if px >= w || py >= h {
                        continue;
                    }
                    let idx = (py as usize) * stride + px as usize;
                    if idx < buf.len() {
                        buf[idx] = color;
                    }
                }
            }
        }
    }
}
