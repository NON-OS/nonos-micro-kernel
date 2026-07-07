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

use crate::clock::fixed::{cos_deg, sin_deg, SCALE};

fn dot(fb: &mut PaintBuffer, x: i32, y: i32, size: u32, argb: u32) {
    if x >= 0 && y >= 0 {
        fb.fill_rect(x as u32, y as u32, size, size, argb);
    }
}

pub fn line(fb: &mut PaintBuffer, x0: i32, y0: i32, x1: i32, y1: i32, w: u32, argb: u32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);
    loop {
        dot(fb, x, y, w, argb);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

pub fn ring(fb: &mut PaintBuffer, cx: i32, cy: i32, r: i32, argb: u32) {
    let mut d = 0;
    while d < 360 {
        let x = cx + r * sin_deg(d) / SCALE;
        let y = cy - r * cos_deg(d) / SCALE;
        dot(fb, x, y, 2, argb);
        d += 2;
    }
}
