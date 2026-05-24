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

use super::metrics::BORDER_PX;

pub fn draw_border(
    pixels: &mut [u32],
    stride_words: usize,
    width: u32,
    height: u32,
    argb: u32,
) {
    fill_band(pixels, stride_words, 0, 0, width, BORDER_PX, argb);
    if height < BORDER_PX {
        return;
    }
    fill_band(pixels, stride_words, 0, height - BORDER_PX, width, BORDER_PX, argb);
    fill_band(pixels, stride_words, 0, 0, BORDER_PX, height, argb);
    if width < BORDER_PX {
        return;
    }
    fill_band(pixels, stride_words, width - BORDER_PX, 0, BORDER_PX, height, argb);
}

fn fill_band(
    pixels: &mut [u32],
    stride_words: usize,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    argb: u32,
) {
    for row in y..y + h {
        let row_start = (row as usize) * stride_words;
        for col in x..x + w {
            let idx = row_start + col as usize;
            if idx < pixels.len() {
                pixels[idx] = argb;
            }
        }
    }
}
