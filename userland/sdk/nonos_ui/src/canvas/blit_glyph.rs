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

use nonos_font::GlyphBitmap;

use super::types::Canvas;

impl Canvas<'_> {
    pub(crate) fn blit_glyph(&mut self, x: u32, y: u32, glyph: &GlyphBitmap, bits: u32) {
        for row in 0..glyph.height as u32 {
            for col in 0..glyph.width as u32 {
                if glyph.rows[row as usize] & (0x80u8 >> col) == 0 {
                    continue;
                }
                let px = x + col;
                let py = y + row;
                if px >= self.width || py >= self.height {
                    continue;
                }
                let index = (py * self.width + px) as usize;
                if index < self.buf.len() {
                    self.buf[index] = bits;
                }
            }
        }
    }
}
