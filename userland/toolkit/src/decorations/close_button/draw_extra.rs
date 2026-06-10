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
use super::fill_box::fill_box;
use super::plot::plot;
use super::types::CloseRect;

pub fn draw_minimize_button(
    pixels: &mut [u32],
    stride_words: usize,
    width: u32,
    rect: &CloseRect,
    fill_argb: u32,
    glyph_argb: u32,
) {
    fill_box(pixels, stride_words, width, rect, fill_argb);
    let row = rect.y + rect.size - 4;
    for i in 3..rect.size - 3 {
        plot(pixels, stride_words, width, rect.x + i, row, glyph_argb);
    }
}

pub fn draw_maximize_button(
    pixels: &mut [u32],
    stride_words: usize,
    width: u32,
    rect: &CloseRect,
    fill_argb: u32,
    glyph_argb: u32,
) {
    fill_box(pixels, stride_words, width, rect, fill_argb);
    let n = rect.size;
    for i in 3..n - 3 {
        plot(pixels, stride_words, width, rect.x + i, rect.y + 3, glyph_argb);
        plot(pixels, stride_words, width, rect.x + i, rect.y + n - 4, glyph_argb);
        plot(pixels, stride_words, width, rect.x + 3, rect.y + i, glyph_argb);
        plot(pixels, stride_words, width, rect.x + n - 4, rect.y + i, glyph_argb);
    }
}
