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

use alloc::vec;
use alloc::vec::Vec;

use super::fill::fill_polys;
use super::math::sqrt;
use super::raster::Raster;

type P = [f32; 2];

// Stroke polylines in device coordinates: each segment fills as a quad of
// the stroke width, each vertex as a square patch standing in for the join
// and cap geometry. At icon scale the difference from round joins is inside
// the antialiasing filter.
pub(super) fn stroke_polys(r: &mut Raster, polys: &[Vec<P>], color: u32, width: f32) {
    let hw = (width / 2.0).max(0.35);
    for poly in polys {
        for pair in poly.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
            let len = sqrt(dx * dx + dy * dy);
            if len < 1e-6 {
                continue;
            }
            let (nx, ny) = (-dy / len * hw, dx / len * hw);
            let quad = vec![
                [a[0] + nx, a[1] + ny],
                [b[0] + nx, b[1] + ny],
                [b[0] - nx, b[1] - ny],
                [a[0] - nx, a[1] - ny],
            ];
            fill_polys(r, &[quad], color, false);
        }
        for &p in poly.iter() {
            let patch = vec![
                [p[0] - hw, p[1] - hw],
                [p[0] + hw, p[1] - hw],
                [p[0] + hw, p[1] + hw],
                [p[0] - hw, p[1] + hw],
            ];
            fill_polys(r, &[patch], color, false);
        }
    }
}
