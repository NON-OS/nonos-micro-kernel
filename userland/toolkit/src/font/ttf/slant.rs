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

use ab_glyph::{point, Font, Glyph, GlyphId, PxScale, ScaleFont};

use super::blend::blend;

// Faux-oblique slant for faces that ship no italic cut: each pixel row is
// pushed right in proportion to its height above the baseline, about 12
// degrees. Only coverage moves; advances and kerning are untouched, so a
// sheared run measures exactly as its upright one does.
pub const OBLIQUE: f32 = 0.22;

// Tracked rendering with the glyph coverage sheared by `slant`; a slant of
// 0.0 is the upright path. The returned pen x still matches `measure_with`,
// so underline rules, hit testing and the caret stay on the drawn glyphs.
#[allow(clippy::too_many_arguments)]
pub fn draw_text_sheared<F: Font>(
    f: &F,
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: i32,
    top_y: i32,
    text: &str,
    argb: u32,
    px: f32,
    spacing: f32,
    slant: f32,
) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    let baseline = top_y as f32 + sf.ascent();
    let mut pen = x as f32;
    let mut prev: Option<GlyphId> = None;
    for ch in text.chars() {
        let mut g: Glyph = sf.scaled_glyph(ch);
        if let Some(p) = prev {
            pen += sf.kern(p, g.id);
        }
        g.position = point(pen, baseline);
        let adv = sf.h_advance(g.id);
        prev = Some(g.id);
        if let Some(og) = sf.outline_glyph(g) {
            let bb = og.px_bounds();
            og.draw(|dx, dy, c| {
                let py = bb.min.y as i32 + dy as i32;
                let shear = (baseline - py as f32) * slant;
                blend(buf, stride, w, h, bb.min.x as i32 + dx as i32 + shear as i32, py, argb, c);
            });
        }
        pen += adv + spacing;
    }
    pen as i32
}
