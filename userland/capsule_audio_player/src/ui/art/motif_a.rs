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

//! The ring, city and horizon motifs from section 04.

use nonos_app_skeleton::PaintBuffer;

use super::curve::{arc_top, dome};
use crate::ui::geometry::Rect;
use crate::ui::paint::ring as ring_stroke;
use crate::ui::theme::alpha;

fn pen(r: &Rect) -> i32 {
    (r.w.min(r.h) / 64).max(1)
}

fn rule(fb: &mut PaintBuffer, r: &Rect, y: i32, c: u32) {
    fb.line_aa(r.x + r.w / 10, r.y + y, r.right() - r.w / 10, r.y + y, c);
}

pub fn ring(fb: &mut PaintBuffer, r: Rect, c: u32) {
    let s = r.w.min(r.h);
    let plane = r.h * 66 / 100;
    let (cx, cy) = (r.cx(), r.y + r.h * 40 / 100);
    ring_stroke(fb, cx, cy, s * 24 / 100, pen(&r) * 2, c);
    rule(fb, &r, plane, alpha(c, 0x66));
    arc_top(fb, cx, r.y + plane + s * 18 / 100, s * 24 / 100, s * 9 / 100, alpha(c, 0x3C));
    let fx = r.x + r.w * 44 / 100;
    let fy = r.y + plane;
    fb.line_aa(fx, fy, fx, fy - s * 9 / 100, alpha(c, 0xCC));
    fb.circle(fx as u32, (fy - s * 11 / 100) as u32, (s / 40).max(1) as u32, alpha(c, 0xCC));
}

pub fn city(fb: &mut PaintBuffer, r: Rect, c: u32) {
    let s = r.w.min(r.h);
    let base = r.y + r.h * 74 / 100;
    fb.circle(r.cx() as u32, (r.y + r.h * 54 / 100) as u32, (s * 19 / 100) as u32, alpha(c, 0x28));
    let heights = [30i32, 46, 22, 58, 38, 50, 26, 42];
    let span = r.w * 76 / 100;
    let bw = span / heights.len() as i32;
    for (i, hv) in heights.iter().enumerate() {
        let x = r.x + r.w * 12 / 100 + i as i32 * bw;
        let top = base - s * hv / 100;
        fb.blend_rect(x as u32, top as u32, (bw - pen(&r) * 2).max(1) as u32, (base - top) as u32, alpha(c, 0x30));
        fb.line_aa(x, top, x + bw - pen(&r) * 2, top, c);
        fb.line_aa(x, top, x, base, alpha(c, 0x9A));
    }
    rule(fb, &r, r.h * 74 / 100, c);
}

pub fn horizon(fb: &mut PaintBuffer, r: Rect, c: u32) {
    let s = r.w.min(r.h);
    let base = r.y + r.h * 70 / 100;
    fb.circle(r.cx() as u32, (r.y + r.h * 44 / 100) as u32, (s * 17 / 100) as u32, alpha(c, 0x44));
    ring_stroke(fb, r.cx(), r.y + r.h * 44 / 100, s * 17 / 100, pen(&r) * 2, c);
    dome(fb, r.x + r.w * 30 / 100, base, r.w * 30 / 100, s * 26 / 100, alpha(c, 0x55));
    dome(fb, r.x + r.w * 72 / 100, base, r.w * 26 / 100, s * 20 / 100, alpha(c, 0x77));
    rule(fb, &r, r.h * 70 / 100, c);
}
