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

//! Box-filtered alpha lookup that resamples a 64px mask to the drawn size.

use super::canvas::Sprite;

fn span(d: u32, n: u32, extent: u32) -> (u32, u32) {
    let lo = d * extent / n;
    let hi = ((d + 1) * extent / n).max(lo + 1).min(extent);
    (lo, hi)
}

pub fn alpha(m: &Sprite, dx: u32, dy: u32, n: u32) -> u32 {
    let (x0, x1) = span(dx, n, m.w);
    let (y0, y1) = span(dy, n, m.h);
    let mut acc = 0u32;
    let mut count = 0u32;
    for sy in y0..y1 {
        for sx in x0..x1 {
            acc += m.rgba[((sy * m.w + sx) * 4 + 3) as usize] as u32;
            count += 1;
        }
    }
    if count == 0 {
        0
    } else {
        acc / count
    }
}
