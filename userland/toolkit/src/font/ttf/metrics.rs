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

use ab_glyph::{Font, GlyphId, PxScale, ScaleFont};

use super::face::face;

// Advance width of `text` at `px`, kerning included. Layout wraps against this
// so painted runs land exactly where layout placed them.
pub fn measure(text: &str, px: f32, mono: bool) -> i32 {
    let Some(f) = face(mono) else { return 0 };
    measure_with(f, text, px)
}

// Same measurement with a caller-provided face, matching draw_text_with.
pub fn measure_with<F: Font>(f: &F, text: &str, px: f32) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    let mut pen = 0.0f32;
    let mut prev: Option<GlyphId> = None;
    for ch in text.chars() {
        let g = sf.glyph_id(ch);
        if let Some(p) = prev {
            pen += sf.kern(p, g);
        }
        pen += sf.h_advance(g);
        prev = Some(g);
    }
    pen as i32
}

// Baseline-to-baseline distance at `px`.
pub fn line_height(px: f32) -> i32 {
    let Some(f) = face(false) else { return px as i32 };
    let sf = f.as_scaled(PxScale::from(px));
    (sf.ascent() - sf.descent() + sf.line_gap()) as i32
}

// Top-of-line to baseline at `px`.
pub fn ascent(px: f32) -> i32 {
    let Some(f) = face(false) else { return px as i32 };
    f.as_scaled(PxScale::from(px)).ascent() as i32
}
