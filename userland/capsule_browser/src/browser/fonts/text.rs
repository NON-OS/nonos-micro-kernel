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

// Advance width in the keyed face at the run's letter-spacing, falling back
// to the built-in faces until the page's font loads, so layout always has a
// metric.
pub fn measure_text(key: u32, mono: bool, text: &str, px: f32, spacing: f32) -> i32 {
    with_face(key, |f| ttf::measure_tracked(f, text, px, spacing))
        .unwrap_or_else(|| ttf::measure_spaced(text, px, mono, spacing))
}

// Draw in the keyed face with the same fallback and spacing the measurement
// used, so the painted glyphs match what layout sized.
pub fn draw_text(fb: &mut PaintBuffer, run: TextRun, text: &str, argb: u32) -> i32 {
    let stride = fb.stride_words as usize;
    let (w, h) = (fb.width, fb.height);
    let TextRun { key, mono, x, top_y, px, spacing } = run;
    let drawn = with_face(key, |f| {
        ttf::draw_text_tracked(f, fb.pixels, stride, w, h, x, top_y, text, argb, px, spacing)
    });
    drawn.unwrap_or_else(|| {
        ttf::draw_text_spaced(fb.pixels, stride, w, h, x, top_y, text, argb, px, mono, spacing)
    })
}

// One text run's face and geometry, grouped so the draw call stays readable.
pub struct TextRun {
    pub key: u32,
    pub mono: bool,
    pub x: i32,
    pub top_y: i32,
    pub px: f32,
    pub spacing: f32,
}
