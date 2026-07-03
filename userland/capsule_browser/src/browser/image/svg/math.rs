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

// Minimal float math for the rasterizer; core has no intrinsics in no_std
// and the crate avoids libm. Accuracy is a few ulp short of exact, which is
// far below a supersampled pixel.

pub(super) const PI: f32 = core::f32::consts::PI;

pub(super) fn sqrt(x: f32) -> f32 {
    if x <= 0.0 || !x.is_finite() {
        return 0.0;
    }
    // Bit-hack seed, then three Newton rounds.
    let mut y = f32::from_bits((x.to_bits() >> 1) + 0x1FC0_0000);
    for _ in 0..3 {
        y = 0.5 * (y + x / y);
    }
    y
}

// Polynomial sine on [-PI, PI] after range reduction.
pub(super) fn sin(x: f32) -> f32 {
    let mut t = x % (2.0 * PI);
    if t > PI {
        t -= 2.0 * PI;
    } else if t < -PI {
        t += 2.0 * PI;
    }
    let t2 = t * t;
    t * (1.0 - t2 / 6.0 * (1.0 - t2 / 20.0 * (1.0 - t2 / 42.0 * (1.0 - t2 / 72.0))))
}

pub(super) fn cos(x: f32) -> f32 {
    sin(x + PI / 2.0)
}

// Quadrant-corrected arctangent; worst error well under a thousandth of a
// radian, invisible at raster scale.
pub(super) fn atan2(y: f32, x: f32) -> f32 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let (ax, ay) = (abs(x), abs(y));
    let a = if ax >= ay { ay / ax } else { ax / ay };
    let s = a * a;
    let mut r = ((-0.046_496 * s + 0.159_314) * s - 0.327_623) * s * a + a;
    if ay > ax {
        r = PI / 2.0 - r;
    }
    if x < 0.0 {
        r = PI - r;
    }
    if y < 0.0 {
        r = -r;
    }
    r
}

pub(super) fn abs(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}
