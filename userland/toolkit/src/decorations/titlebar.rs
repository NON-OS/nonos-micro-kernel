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

use super::metrics::{TITLEBAR_HEIGHT, TITLEBAR_PADDING, TITLE_TEXT_Y};
use crate::font::render::draw_text;

pub fn draw_titlebar(
    pixels: &mut [u32],
    stride_words: usize,
    width: u32,
    height: u32,
    band_argb: u32,
    text_argb: u32,
    title: &[u8],
) {
    if height < TITLEBAR_HEIGHT {
        return;
    }
    for row in 0..TITLEBAR_HEIGHT {
        let row_start = (row as usize) * stride_words;
        for col in 0..width {
            let idx = row_start + col as usize;
            if idx < pixels.len() {
                pixels[idx] = band_argb;
            }
        }
    }
    draw_text(
        pixels,
        stride_words,
        width,
        height,
        TITLEBAR_PADDING,
        TITLE_TEXT_Y,
        title,
        text_argb,
    );
}
