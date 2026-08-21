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

//! Unit-circle steps for icon arcs.
//!
//! 64 steps around the circle, quarter table mirrored into the other three, so a
//! curve is described by a centre, radii and a sweep rather than by hand-listed
//! points. Hand-listed curves were the reason the old globe and Wi-Fi arcs read
//! as polygons. Separate `rx`/`ry` gives ellipses, which is what a drum lid is.

pub const STEPS: i32 = 64;

const SIN_Q: [f32; 17] = [
    0.0, 0.098_017, 0.195_090, 0.290_285, 0.382_683, 0.471_397, 0.555_570, 0.634_393, 0.707_107,
    0.773_010, 0.831_470, 0.881_921, 0.923_880, 0.956_940, 0.980_785, 0.995_185, 1.0,
];

pub fn unit(step: i32) -> (f32, f32) {
    let s = step.rem_euclid(STEPS);
    let (q, i) = (s / 16, (s % 16) as usize);
    let (cos, sin) = (SIN_Q[16 - i], SIN_Q[i]);
    match q {
        0 => (cos, sin),
        1 => (-sin, cos),
        2 => (-cos, -sin),
        _ => (sin, -cos),
    }
}
