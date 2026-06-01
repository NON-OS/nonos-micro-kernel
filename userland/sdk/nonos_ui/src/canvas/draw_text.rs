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

use nonos_font::{glyph_for_ascii, GLYPH_WIDTH, LETTER_SPACING};

use super::types::Canvas;
use crate::color::Color;

impl Canvas<'_> {
    pub fn draw_text(&mut self, x: u32, y: u32, text: &str, color: Color) {
        let bits = color.bits();
        let mut pen = x;
        for ch in text.chars() {
            let ascii = if (ch as u32) < 128 { ch as u8 } else { 0 };
            let glyph = glyph_for_ascii(ascii);
            self.blit_glyph(pen, y, glyph, bits);
            pen = pen.saturating_add(GLYPH_WIDTH + LETTER_SPACING);
        }
    }
}
