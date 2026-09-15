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

//! Integer curve sampling for the cover motifs. No float trigonometry exists
//! under the user target, so a quarter-period sine table plus the toolkit's
//! integer square root carry every rounded contour a motif needs.

use nonos_app_skeleton::paint::radius::isqrt;
use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;

const QUARTER: [i32; 65] = [
    0, 25, 49, 74, 98, 122, 147, 171, 195, 219, 243, 267, 290,
    314, 337, 360, 383, 405, 428, 450, 471, 493, 514, 535, 556, 576,
    596, 615, 634, 653, 672, 690, 707, 724, 741, 757, 773, 788, 803,
    818, 831, 845, 858, 870, 882, 893, 904, 914, 924, 933, 942, 950,
    957, 964, 970, 976, 981, 985, 989, 992, 995, 997, 999, 1000, 1000,
];

pub fn sin(deg: i32) -> i32 {
    let d = deg.rem_euclid(360);
    match d {
        0..=89 => QUARTER[(d * 64 / 90) as usize],
        90..=179 => QUARTER[((180 - d) * 64 / 90) as usize],
        180..=269 => -QUARTER[((d - 180) * 64 / 90) as usize],
        _ => -QUARTER[((360 - d) * 64 / 90) as usize],
    }
}

pub fn cos(deg: i32) -> i32 {
    sin(deg + 90)
}

pub fn ellipse(fb: &mut PaintBuffer, cx: i32, cy: i32, a: i32, b: i32, argb: u32) {
    if a <= 0 || b <= 0 {
        return;
    }
    let mut prev = (cx + a, cy);
    let mut t = 6;
    while t <= 360 {
        let p = (cx + a * cos(t) / 1000, cy + b * sin(t) / 1000);
        fb.line_aa(prev.0, prev.1, p.0, p.1, argb);
        prev = p;
        t += 6;
    }
}

pub fn arc_top(fb: &mut PaintBuffer, cx: i32, cy: i32, a: i32, b: i32, argb: u32) {
    let mut prev = (cx - a, cy);
    let mut t = 186;
    while t <= 354 {
        let p = (cx + a * cos(t) / 1000, cy + b * sin(t) / 1000);
        fb.line_aa(prev.0, prev.1, p.0, p.1, argb);
        prev = p;
        t += 6;
    }
}

pub fn contour(fb: &mut PaintBuffer, r: Rect, y: i32, amp: i32, cycles: i32, phase: i32, argb: u32) {
    let steps = (r.w / 3).max(8);
    let mut prev = (r.x, r.y + y);
    for i in 1..=steps {
        let x = r.x + r.w * i / steps;
        let deg = phase + 360 * cycles * i / steps;
        let py = r.y + y + amp * sin(deg) / 1000;
        fb.line_aa(prev.0, prev.1, x, py, argb);
        prev = (x, py);
    }
}

pub fn dome(fb: &mut PaintBuffer, cx: i32, base: i32, half: i32, rise: i32, argb: u32) {
    let mut prev = (cx - half, base);
    for i in 1..=16 {
        let x = cx - half + half * 2 * i / 16;
        let dx = (x - cx).abs();
        let k = isqrt(((half * half - dx * dx).max(0)) as u64) as i32;
        let y = base - rise * k / half.max(1);
        fb.line_aa(prev.0, prev.1, x, y, argb);
        prev = (x, y);
    }
}
