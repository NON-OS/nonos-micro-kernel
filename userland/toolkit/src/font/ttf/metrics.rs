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
use super::readable::readable_px;

// Advance width of `text` at `px`, kerning included. Layout wraps against this
// so painted runs land exactly where layout placed them. The size is clamped to
// the readable floor, the same clamp draw_text applies, so measured runs match
// what is painted.
pub fn measure(text: &str, px: f32, mono: bool) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(mono) else { return 0 };
    measure_with(f, text, px)
}

// Same measurement with a caller-provided face, matching draw_text_with.
pub fn measure_with<F: Font>(f: &F, text: &str, px: f32) -> i32 {
    measure_tracked(f, text, px, 0.0)
}

// Advance width with extra spacing per glyph, matching draw_text_tracked.
pub fn measure_tracked<F: Font>(f: &F, text: &str, px: f32, spacing: f32) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    let mut pen = 0.0f32;
    let mut prev: Option<GlyphId> = None;
    for ch in text.chars() {
        let g = sf.glyph_id(ch);
        if let Some(p) = prev {
            pen += sf.kern(p, g);
        }
        pen += sf.h_advance(g) + spacing;
        prev = Some(g);
    }
    pen as i32
}

// Tracked measurement with the built-in faces.
pub fn measure_spaced(text: &str, px: f32, mono: bool, spacing: f32) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(mono) else { return 0 };
    measure_tracked(f, text, px, spacing)
}

// Baseline-to-baseline distance at `px`.
pub fn line_height(px: f32) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(false) else { return px as i32 };
    let sf = f.as_scaled(PxScale::from(px));
    (sf.ascent() - sf.descent() + sf.line_gap()) as i32
}

// Same measurement with a caller-provided face, matching measure_with. Unlike
// line_height, px is not clamped to the readable floor.
pub fn line_height_with<F: Font>(f: &F, px: f32) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    (sf.ascent() - sf.descent() + sf.line_gap()) as i32
}

// Top-of-line to baseline at `px`.
pub fn ascent(px: f32) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(false) else { return px as i32 };
    f.as_scaled(PxScale::from(px)).ascent() as i32
}

// Same measurement with a caller-provided face, matching measure_with. Unlike
// ascent, px is not clamped to the readable floor.
pub fn ascent_with<F: Font>(f: &F, px: f32) -> i32 {
    f.as_scaled(PxScale::from(px)).ascent() as i32
}
