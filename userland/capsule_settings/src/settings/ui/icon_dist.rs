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

pub const SUB: i32 = 4;
pub const STEP: f32 = 1.0 / SUB as f32;

pub fn span(a: f32, b: f32, half: f32, dim: u32) -> (u32, u32) {
    let lo = if a < b { a } else { b } - half - 1.0;
    let hi = if a > b { a } else { b } + half + 1.0;
    let lo = if lo < 0.0 { 0 } else { lo as u32 };
    (lo.min(dim), ((hi as i32).max(0) as u32 + 1).min(dim))
}

pub fn dist2(px: f32, py: f32, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let (dx, dy) = (x1 - x0, y1 - y0);
    let len2 = dx * dx + dy * dy;
    let mut t = 0.0;
    if len2 > 0.0 {
        t = ((px - x0) * dx + (py - y0) * dy) / len2;
        if t < 0.0 {
            t = 0.0;
        } else if t > 1.0 {
            t = 1.0;
        }
    }
    let (qx, qy) = (x0 + t * dx, y0 + t * dy);
    (px - qx) * (px - qx) + (py - qy) * (py - qy)
}
