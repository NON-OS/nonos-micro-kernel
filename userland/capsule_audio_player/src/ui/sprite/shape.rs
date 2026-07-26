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

//! Triangle and rounded-rectangle coverage fills.

use super::canvas::Sprite;
use super::prim::{coverage, SS};

pub fn tri(s: &mut Sprite, p: [(i32, i32); 3], rgb: u32) {
    let v = [(p[0].0 * SS, p[0].1 * SS), (p[1].0 * SS, p[1].1 * SS), (p[2].0 * SS, p[2].1 * SS)];
    for y in 0..s.h {
        for x in 0..s.w {
            let a = coverage(x, y, |px, py| in_tri(v, px, py));
            if a > 0 {
                s.set(x, y, rgb, a);
            }
        }
    }
}

fn edge(a: (i32, i32), b: (i32, i32), px: i32, py: i32) -> i64 {
    (b.0 - a.0) as i64 * (py - a.1) as i64 - (b.1 - a.1) as i64 * (px - a.0) as i64
}

fn in_tri(v: [(i32, i32); 3], px: i32, py: i32) -> bool {
    let (d0, d1, d2) = (edge(v[0], v[1], px, py), edge(v[1], v[2], px, py), edge(v[2], v[0], px, py));
    (d0 >= 0 && d1 >= 0 && d2 >= 0) || (d0 <= 0 && d1 <= 0 && d2 <= 0)
}

pub fn rrect(s: &mut Sprite, x0: i32, y0: i32, w: i32, h: i32, rad: i32, rgb: u32) {
    for y in 0..s.h {
        for x in 0..s.w {
            let a = coverage(x, y, |px, py| in_rrect(px, py, x0 * SS, y0 * SS, w * SS, h * SS, rad * SS));
            if a > 0 {
                s.set(x, y, rgb, a);
            }
        }
    }
}

fn in_rrect(px: i32, py: i32, x0: i32, y0: i32, w: i32, h: i32, r: i32) -> bool {
    if px < x0 || py < y0 || px >= x0 + w || py >= y0 + h {
        return false;
    }
    let cx = px.clamp(x0 + r, x0 + w - r);
    let cy = py.clamp(y0 + r, y0 + h - r);
    let (dx, dy) = (px - cx, py - cy);
    dx * dx + dy * dy <= r * r
}
