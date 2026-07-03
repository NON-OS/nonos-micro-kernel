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

use super::math::sqrt;

// Row-major 2x3 affine: x' = a*x + c*y + e, y' = b*x + d*y + f.
#[derive(Clone, Copy)]
pub(super) struct Affine(pub [f32; 6]);

impl Affine {
    pub fn identity() -> Self {
        Affine([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }

    pub fn translate(tx: f32, ty: f32) -> Self {
        Affine([1.0, 0.0, 0.0, 1.0, tx, ty])
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        Affine([sx, 0.0, 0.0, sy, 0.0, 0.0])
    }

    // self ∘ rhs: rhs applies first.
    pub fn then(&self, rhs: &Affine) -> Affine {
        let a = self.0;
        let b = rhs.0;
        Affine([
            a[0] * b[0] + a[2] * b[1],
            a[1] * b[0] + a[3] * b[1],
            a[0] * b[2] + a[2] * b[3],
            a[1] * b[2] + a[3] * b[3],
            a[0] * b[4] + a[2] * b[5] + a[4],
            a[1] * b[4] + a[3] * b[5] + a[5],
        ])
    }

    pub fn apply(&self, p: [f32; 2]) -> [f32; 2] {
        let m = self.0;
        [m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5]]
    }

    // Average absolute scale, for stroke widths under non-uniform scaling.
    pub fn scale_avg(&self) -> f32 {
        let m = self.0;
        (sqrt(m[0] * m[0] + m[1] * m[1]) + sqrt(m[2] * m[2] + m[3] * m[3])) / 2.0
    }
}
