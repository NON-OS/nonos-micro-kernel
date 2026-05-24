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

use super::metrics::{CLOSE_BUTTON_SIZE, TITLEBAR_HEIGHT, TITLEBAR_PADDING};

pub struct CloseRect {
    pub x: u32,
    pub y: u32,
    pub size: u32,
}

pub fn close_button_rect(width: u32) -> CloseRect {
    let pad = TITLEBAR_PADDING;
    let size = CLOSE_BUTTON_SIZE;
    let y = (TITLEBAR_HEIGHT - size) / 2;
    let x = width.saturating_sub(pad + size);
    CloseRect { x, y, size }
}

pub fn draw_close_button(
    pixels: &mut [u32],
    stride_words: usize,
    width: u32,
    rect: &CloseRect,
    fill_argb: u32,
    glyph_argb: u32,
) {
    fill_box(pixels, stride_words, width, rect.x, rect.y, rect.size, rect.size, fill_argb);
    let n = rect.size;
    for i in 2..n - 2 {
        plot(pixels, stride_words, width, rect.x + i, rect.y + i, glyph_argb);
        plot(pixels, stride_words, width, rect.x + (n - 1 - i), rect.y + i, glyph_argb);
    }
}

fn fill_box(pixels: &mut [u32], stride_words: usize, width: u32, x: u32, y: u32, w: u32, h: u32, argb: u32) {
    for row in y..y + h {
        let base = (row as usize) * stride_words;
        for col in x..x + w {
            let idx = base + col as usize;
            if col < width && idx < pixels.len() {
                pixels[idx] = argb;
            }
        }
    }
}

fn plot(pixels: &mut [u32], stride_words: usize, width: u32, x: u32, y: u32, argb: u32) {
    if x >= width {
        return;
    }
    let idx = (y as usize) * stride_words + x as usize;
    if idx < pixels.len() {
        pixels[idx] = argb;
    }
}
