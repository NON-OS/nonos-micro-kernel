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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf;

use super::registry::with_face;

// Advance width in the keyed face, falling back to the built-in faces until
// the page's font loads, so layout always has a metric.
pub fn measure_text(key: u32, mono: bool, text: &str, px: f32) -> i32 {
    with_face(key, |f| ttf::measure_with(f, text, px)).unwrap_or_else(|| {
        if mono {
            nonos_app_skeleton::measure_ttf_mono(text, px)
        } else {
            nonos_app_skeleton::measure_ttf(text, px)
        }
    })
}

// Draw in the keyed face with the same fallback the measurement used, so the
// painted glyphs match what layout sized.
pub fn draw_text(
    fb: &mut PaintBuffer,
    key: u32,
    mono: bool,
    x: i32,
    top_y: i32,
    text: &str,
    argb: u32,
    px: f32,
) -> i32 {
    let stride = fb.stride_words as usize;
    let (w, h) = (fb.width, fb.height);
    let drawn = with_face(key, |f| {
        ttf::draw_text_with(f, fb.pixels, stride, w, h, x, top_y, text, argb, px)
    });
    drawn.unwrap_or_else(|| {
        if mono {
            fb.text_ttf_mono(x, top_y, text, argb, px)
        } else {
            fb.text_ttf(x, top_y, text, argb, px)
        }
    })
}
