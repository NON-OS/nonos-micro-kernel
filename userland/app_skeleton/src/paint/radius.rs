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


pub fn isqrt(v: u64) -> u32 {
    if v == 0 {
        return 0;
    }
    let mut x = v;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + v / x) / 2;
    }
    x as u32
}

pub fn clamp_radius(w: u32, h: u32, r: u32) -> u32 {
    let limit = if w < h { w } else { h } / 2;
    if r < limit {
        r
    } else {
        limit
    }
}

fn corner_off(p: u32, span: u32, r: u32) -> u32 {
    let c = p * 256 + 128;
    let edge = r * 256;
    let near = edge.saturating_sub(c);
    let far = (edge + c).saturating_sub(span * 256);
    if near > far {
        near
    } else {
        far
    }
}

pub fn coverage(px: u32, py: u32, w: u32, h: u32, r: u32) -> u32 {
    if r == 0 || px >= w || py >= h {
        return 255;
    }
    let dx = corner_off(px, w, r) as u64;
    let dy = corner_off(py, h, r) as u64;
    if dx == 0 || dy == 0 {
        return 255;
    }
    let dist = isqrt(dx * dx + dy * dy);
    let v = (r * 256 + 128).saturating_sub(dist);
    if v > 255 {
        255
    } else {
        v
    }
}
