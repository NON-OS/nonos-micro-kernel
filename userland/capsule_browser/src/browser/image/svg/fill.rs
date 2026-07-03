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

use alloc::vec::Vec;

use super::raster::Raster;

type P = [f32; 2];

// Scanline fill of a set of subpaths already in device coordinates. Each
// subpath closes implicitly. Crossings at the sample row carry a winding
// direction so both fill rules evaluate from the same list.
pub(super) fn fill_polys(r: &mut Raster, polys: &[Vec<P>], color: u32, evenodd: bool) {
    if polys.is_empty() {
        return;
    }
    // Only rows the geometry can touch.
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;
    for poly in polys {
        for p in poly {
            y_min = y_min.min(p[1]);
            y_max = y_max.max(p[1]);
        }
    }
    let y0 = (y_min as i32).max(0);
    let y1 = ((y_max + 1.0) as i32).min(r.h as i32);
    let mut xs: Vec<(f32, i32)> = Vec::new();
    for y in y0..y1 {
        let sy = y as f32 + 0.5;
        xs.clear();
        for poly in polys {
            let n = poly.len();
            for i in 0..n {
                let a = poly[i];
                let b = poly[(i + 1) % n];
                if (a[1] <= sy) == (b[1] <= sy) {
                    continue;
                }
                let t = (sy - a[1]) / (b[1] - a[1]);
                let x = a[0] + t * (b[0] - a[0]);
                xs.push((x, if b[1] > a[1] { 1 } else { -1 }));
            }
        }
        if xs.is_empty() {
            continue;
        }
        xs.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap_or(core::cmp::Ordering::Equal));
        let mut wind = 0i32;
        let mut inside = false;
        let mut span_start = 0f32;
        for &(x, dir) in xs.iter() {
            let was = inside;
            if evenodd {
                inside = !inside;
            } else {
                wind += dir;
                inside = wind != 0;
            }
            if !was && inside {
                span_start = x;
            } else if was && !inside {
                let x0 = (span_start + 0.5) as i32;
                let x1 = (x + 0.5) as i32;
                for px in x0.max(0)..x1.min(r.w as i32) {
                    r.blend(px, y, color);
                }
            }
        }
    }
}
