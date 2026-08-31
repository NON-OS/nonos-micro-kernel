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

fn isqrt(v: u32) -> u32 {
    let mut root = 0u32;
    while (root + 1) * (root + 1) <= v {
        root += 1;
    }
    root
}

fn corner_depth(row: u32, h: u32, r: u32) -> u32 {
    let from_bottom = h.saturating_sub(1).saturating_sub(row);
    if row < r {
        r - row
    } else if from_bottom < r {
        r - from_bottom
    } else {
        0
    }
}

pub fn clamp_radius(w: u32, h: u32, r: u32) -> u32 {
    let limit = if w < h { w } else { h } / 2;
    if r < limit {
        r
    } else {
        limit
    }
}

pub fn inset(row: u32, h: u32, r: u32) -> u32 {
    if r == 0 || row >= h {
        return 0;
    }
    let depth = corner_depth(row, h, r);
    if depth == 0 {
        return 0;
    }
    r - isqrt(r * r - depth * depth)
}
