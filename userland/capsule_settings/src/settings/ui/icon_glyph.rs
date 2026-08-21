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

use super::icon_arc::unit;
use super::icon_mask::Mask;

/// An arc on the 20x20 icon grid: centre, radii, and a sweep measured in
/// sixty-fourths of a turn from `from`. Equal radii give a circle.
pub struct Arc {
    pub cx: i8,
    pub cy: i8,
    pub rx: i8,
    pub ry: i8,
    pub from: i8,
    pub sweep: i8,
}

pub struct Glyph {
    pub strokes: &'static [&'static [(i8, i8)]],
    pub arcs: &'static [Arc],
    /// Arcs swept from the centre outwards, so a half-turn wedge fills a half
    /// disc. The mask has no polygon fill and a scanline one would want a square
    /// root, which `#![no_std]` does not hand us.
    pub wedges: &'static [Arc],
    pub dots: &'static [(i8, i8)],
}

const GRID: f32 = 20.0;

pub fn draw_glyph(fb: &mut PaintBuffer, g: &Glyph, x: u32, y: u32, size: u32, argb: u32) {
    let mut mask = Mask::new(size);
    let s = mask.dim() as f32;
    let half = (s / 24.0).max(0.7);
    let at = |v: i8| v as f32 * s / GRID;
    for line in g.strokes {
        for w in line.windows(2) {
            mask.segment(at(w[0].0), at(w[0].1), at(w[1].0), at(w[1].1), half);
        }
    }
    for a in g.arcs {
        trace_arc(&mut mask, a, s, half);
    }
    for w in g.wedges {
        fill_wedge(&mut mask, w, s, half);
    }
    for d in g.dots {
        mask.segment(at(d.0), at(d.1), at(d.0), at(d.1), half * 1.9);
    }
    blend_mask(fb, &mask, x, y, argb);
}

fn trace_arc(mask: &mut Mask, a: &Arc, s: f32, half: f32) {
    let k = s / GRID;
    let (cx, cy) = (a.cx as f32 * k, a.cy as f32 * k);
    let (rx, ry) = (a.rx as f32 * k, a.ry as f32 * k);
    let steps = a.sweep as i32;
    let mut prev = None;
    for i in 0..=steps.abs() {
        let step = a.from as i32 + if steps < 0 { -i } else { i };
        let (c, sn) = unit(step);
        let p = (cx + c * rx, cy - sn * ry);
        if let Some((px, py)) = prev {
            mask.segment(px, py, p.0, p.1, half);
        }
        prev = Some(p);
    }
}

fn fill_wedge(mask: &mut Mask, a: &Arc, s: f32, half: f32) {
    let outer = if a.rx > a.ry { a.rx } else { a.ry } as f32 * s / GRID;
    let mut r = half;
    while r < outer {
        let k = r / outer;
        let ring = Arc {
            cx: a.cx,
            cy: a.cy,
            rx: (a.rx as f32 * k) as i8,
            ry: (a.ry as f32 * k) as i8,
            from: a.from,
            sweep: a.sweep,
        };
        trace_arc(mask, &ring, s, half);
        r += half;
    }
    trace_arc(mask, a, s, half);
}

fn blend_mask(fb: &mut PaintBuffer, mask: &Mask, x: u32, y: u32, argb: u32) {
    let base = argb & 0x00FF_FFFF;
    let src_a = (argb >> 24) as u32;
    for my in 0..mask.dim() {
        for mx in 0..mask.dim() {
            let cover = mask.at(mx, my) as u32;
            if cover == 0 {
                continue;
            }
            let a = cover * src_a / 255;
            fb.blend_px(x + mx, y + my, (a << 24) | base);
        }
    }
}
