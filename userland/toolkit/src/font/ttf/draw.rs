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
use super::face::face;

// Render `text` with its top-left at (x, top_y) and return the pen x after
// the last glyph. `px` is the em size in pixels. Kerning is applied between
// adjacent glyphs; each glyph is rasterized and alpha-composited.
pub fn draw_text(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: i32,
    top_y: i32,
    text: &str,
    argb: u32,
    px: f32,
    mono: bool,
) -> i32 {
    let Some(f) = face(mono) else { return x };
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
                blend(
                    buf,
                    stride,
                    w,
                    h,
                    bb.min.x as i32 + dx as i32,
                    bb.min.y as i32 + dy as i32,
                    argb,
                    c,
                );
            });
        }
        pen += adv;
    }
    pen as i32
}
