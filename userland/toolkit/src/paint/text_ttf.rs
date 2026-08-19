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

use crate::font::ttf;

use super::buffer::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    // Antialiased body text. `top_y` is the top of the line box; `px` the em
    // size. Returns the pen x after the last glyph so callers can chain runs.
    pub fn text_ttf(&mut self, x: i32, top_y: i32, text: &str, argb: u32, px: f32) -> i32 {
        ttf::draw_text(
            self.pixels,
            self.stride_words as usize,
            self.width,
            self.height,
            x,
            top_y,
            text,
            argb,
            px,
            false,
        )
    }

    // Same as `text_ttf` but rendered with the monospace face, for pre/code.
    pub fn text_ttf_mono(&mut self, x: i32, top_y: i32, text: &str, argb: u32, px: f32) -> i32 {
        ttf::draw_text(
            self.pixels,
            self.stride_words as usize,
            self.width,
            self.height,
            x,
            top_y,
            text,
            argb,
            px,
            true,
        )
    }

    // Advance width of `text` at `px`, matching what `text_ttf` will paint.
    pub fn measure_ttf(&self, text: &str, px: f32) -> i32 {
        ttf::measure(text, px, false)
    }

    // Advance width in the monospace face, matching `text_ttf_mono`.
    pub fn measure_ttf_mono(&self, text: &str, px: f32) -> i32 {
        ttf::measure(text, px, true)
    }
}

// Surface-free measurement for layout, which sizes runs before it owns a
// PaintBuffer. `mono` selects the same face the paint pass will use.
pub fn measure_ttf(text: &str, px: f32) -> i32 {
    ttf::measure(text, px, false)
}

pub fn measure_ttf_mono(text: &str, px: f32) -> i32 {
    ttf::measure(text, px, true)
}
