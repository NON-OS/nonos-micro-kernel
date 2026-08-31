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

use ab_glyph::{point, Font, FontRef, GlyphId, PxScale, ScaleFont};

use super::blend::blend;
use super::cache::{self, Raster};
use super::face::face;
use super::readable::readable_px;

// Render `text` with its top-left at (x, top_y) and return the pen x after
// the last glyph. `px` is the em size in pixels. Kerning is applied between
// adjacent glyphs; each glyph is rasterized once and cached, then blended.
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
    let px = readable_px(px);
    let Some(f) = face(mono) else { return x };
    draw_cached(f, mono, buf, stride, w, h, x, top_y, text, argb, px)
}

// The hot path (body, gutter, labels): rasterize each glyph once into the cache,
// then blend the cached coverage in the requested colour. This keeps a full
// screen of crisp text cheap enough to repaint on every keystroke.
#[allow(clippy::too_many_arguments)]
fn draw_cached(
    f: &FontRef,
    mono: bool,
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: i32,
    top_y: i32,
    text: &str,
    argb: u32,
    px: f32,
) -> i32 {
    let sf = f.as_scaled(PxScale::from(px));
    let baseline = top_y as f32 + sf.ascent();
    let px_bits = px.to_bits();
    let mut pen = x as f32;
    let mut prev: Option<GlyphId> = None;
    for ch in text.chars() {
        let g = sf.scaled_glyph(ch);
        if let Some(p) = prev {
            pen += sf.kern(p, g.id);
        }
        let adv = sf.h_advance(g.id);
        let gid = g.id.0;
        prev = Some(g.id);
        let pen_i = pen as i32;
        let base_i = baseline as i32;
        cache::with_raster(
            (mono, gid, px_bits),
            || {
                // Rasterize the glyph at the pen origin so the coverage is
                // position independent and reusable at any pen x.
                let mut og_glyph = sf.scaled_glyph(ch);
                og_glyph.position = point(0.0, 0.0);
                let og = sf.outline_glyph(og_glyph)?;
                let bb = og.px_bounds();
                let min_x = floor_i32(bb.min.x);
                let min_y = floor_i32(bb.min.y);
                let gw = (ceil_i32(bb.max.x) - min_x).max(0) as u32;
                let gh = (ceil_i32(bb.max.y) - min_y).max(0) as u32;
                if gw == 0 || gh == 0 {
                    return None;
                }
                let mut cov = alloc::vec![0u8; (gw * gh) as usize];
                og.draw(|dx, dy, c| {
                    let idx = (dy * gw + dx) as usize;
                    if idx < cov.len() {
                        cov[idx] = (c * 255.0) as u8;
                    }
                });
                Some(Raster { min_x, min_y, w: gw, h: gh, cov })
            },
            |r| blit_raster(r, buf, stride, w, h, pen_i, base_i, argb),
        );
        pen += adv;
    }
    pen as i32
}

// no_std floor/ceil for the small positive-ish values glyph bounds produce.
fn floor_i32(x: f32) -> i32 {
    let i = x as i32;
    if (i as f32) > x {
        i - 1
    } else {
        i
    }
}

fn ceil_i32(x: f32) -> i32 {
    let i = x as i32;
    if (i as f32) < x {
        i + 1
    } else {
        i
    }
}

#[allow(clippy::too_many_arguments)]
fn blit_raster(
    r: &Raster,
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    pen_x: i32,
    baseline: i32,
    argb: u32,
) {
    for dy in 0..r.h {
        let row = (dy * r.w) as usize;
        for dx in 0..r.w {
            let a = r.cov[row + dx as usize];
            if a != 0 {
                let px = pen_x + r.min_x + dx as i32;
                let py = baseline + r.min_y + dy as i32;
                blend(buf, stride, w, h, px, py, argb, a as f32 / 255.0);
            }
        }
    }
}

// Same rendering with a caller-provided face, so text can draw in a font
// loaded at runtime, such as a page's web font.
pub fn draw_text_with<F: Font>(
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
) -> i32 {
    draw_text_tracked(f, buf, stride, w, h, x, top_y, text, argb, px, 0.0)
}

// Same rendering with extra advance between glyphs, for letter-spacing.
#[allow(clippy::too_many_arguments)]
pub fn draw_text_tracked<F: Font>(
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
) -> i32 {
    super::slant::draw_text_sheared(f, buf, stride, w, h, x, top_y, text, argb, px, spacing, 0.0)
}

// Tracked rendering with the built-in faces, the fallback while a page font
// is still loading.
pub fn draw_text_spaced(
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
    spacing: f32,
) -> i32 {
    let px = readable_px(px);
    let Some(f) = face(mono) else { return x };
    draw_text_tracked(f, buf, stride, w, h, x, top_y, text, argb, px, spacing)
}
