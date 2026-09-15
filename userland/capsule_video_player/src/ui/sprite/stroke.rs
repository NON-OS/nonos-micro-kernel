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

//! Anti-aliased thick line strokes via segment-distance coverage.

use super::canvas::Sprite;
use super::prim::{coverage, SS};

pub fn line(s: &mut Sprite, a: (i32, i32), b: (i32, i32), th: i32, rgb: u32) {
    let (a, b) = ((a.0 * SS, a.1 * SS), (b.0 * SS, b.1 * SS));
    let half = (th * SS / 2) as i64;
    let tt = half * half;
    for y in 0..s.h {
        for x in 0..s.w {
            let cov = coverage(x, y, |px, py| seg_d2(a, b, px, py) <= tt);
            if cov > 0 {
                s.set(x, y, rgb, cov);
            }
        }
    }
}

fn seg_d2(a: (i32, i32), b: (i32, i32), px: i32, py: i32) -> i64 {
    let (vx, vy) = ((b.0 - a.0) as i64, (b.1 - a.1) as i64);
    let (wx, wy) = ((px - a.0) as i64, (py - a.1) as i64);
    let c1 = vx * wx + vy * wy;
    if c1 <= 0 {
        return wx * wx + wy * wy;
    }
    let c2 = vx * vx + vy * vy;
    if c2 <= c1 {
        let (ex, ey) = ((px - b.0) as i64, (py - b.1) as i64);
        return ex * ex + ey * ey;
    }
    ((wx * wx + wy * wy) * c2 - c1 * c1) / c2
}
