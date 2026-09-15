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

use super::icon_path::{Icon, Pt};
use super::icon_path_more::glyph;

/// Strokes `i` into the `size`-square box at `(x, y)` in `color`. Every mark is
/// a blending primitive, so this is safe over paint the capsule already laid
/// down on the transparent overlay.
pub fn draw(fb: &mut PaintBuffer, i: Icon, x: u32, y: u32, size: u32, color: u32) {
    let g = glyph(i);
    for poly in g.polys {
        let mut prev: Option<(i32, i32)> = None;
        for p in poly.iter() {
            let cur = map(x, y, size, *p);
            if let Some(a) = prev {
                stroke(fb, a, cur, color);
            }
            prev = Some(cur);
        }
    }
    let t = weight(size);
    for (cx, cy, r) in g.rings {
        let (px, py) = map(x, y, size, (*cx, *cy));
        let rr = ((*r as u32) * size / 1000).max(1);
        fb.ring(px.max(0) as u32, py.max(0) as u32, rr, t, color);
    }
}

fn weight(size: u32) -> u32 {
    (size / 9).max(1)
}

fn map(x: u32, y: u32, size: u32, p: Pt) -> (i32, i32) {
    let s = size as i32;
    (x as i32 + (p.0 as i32 * s) / 1000, y as i32 + (p.1 as i32 * s) / 1000)
}

fn stroke(fb: &mut PaintBuffer, a: (i32, i32), b: (i32, i32), color: u32) {
    fb.line_aa(a.0, a.1, b.0, b.1, color);
    let (ox, oy) = if (b.0 - a.0).abs() >= (b.1 - a.1).abs() { (0, 1) } else { (1, 0) };
    fb.line_aa(a.0 + ox, a.1 + oy, b.0 + ox, b.1 + oy, color);
}
