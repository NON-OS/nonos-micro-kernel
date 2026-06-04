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
use crate::font::atlas::FontAtlas;

use super::draw_glyph::draw_glyph;

pub fn draw_text(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    text: &[u8],
    color: u32,
) {
    let atlas = FontAtlas::default();
    let mut pen_x = x;
    for &ch in text {
        let glyph = atlas.glyph(ch);
        draw_glyph(buf, stride, w, h, pen_x, y, glyph, color);
        pen_x = pen_x.saturating_add(atlas.glyph_width as u32 + atlas.letter_spacing as u32);
    }
}
