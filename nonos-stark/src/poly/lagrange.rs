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

//! Lagrange evaluation of the low-degree extension. Given a set of points, this
//! evaluates the unique interpolating polynomial of degree below the point count
//! at an arbitrary location, without materializing its coefficients. This is the
//! operation a STARK uses to move between a trace and its extension.

use super::super::field::{Fp, Fp2};

/// Evaluate the polynomial that interpolates `(xs[i], ys[i])` at `z`. The `xs`
/// must be distinct; a repeated node makes the interpolant undefined and the
/// corresponding denominator zero, so callers pass distinct evaluation points.
pub fn eval_lagrange(xs: &[Fp], ys: &[Fp], z: Fp) -> Fp {
    let n = core::cmp::min(xs.len(), ys.len());
    let mut acc = Fp::ZERO;
    for i in 0..n {
        let mut num = Fp::ONE;
        let mut den = Fp::ONE;
        for j in 0..n {
            if i != j {
                num = num * (z - xs[j]);
                den = den * (xs[i] - xs[j]);
            }
        }
        acc = acc + ys[i] * num * den.inv();
    }
    acc
}

/// Evaluate the same interpolant at an extension point `z in Fp2`. The nodes and
/// values stay in the base field (a periodic column is public and base-valued);
/// only the evaluation point is in the extension, as the out-of-domain sampling of
/// a money-grade STARK requires. Agrees with `eval_lagrange` on a base-embedded z.
pub fn eval_lagrange_ext(xs: &[Fp], ys: &[Fp], z: Fp2) -> Fp2 {
    let n = core::cmp::min(xs.len(), ys.len());
    let mut acc = Fp2::ZERO;
    for i in 0..n {
        let mut num = Fp2::ONE;
        let mut den = Fp::ONE;
        for j in 0..n {
            if i != j {
                num = num * (z - Fp2::from_base(xs[j]));
                den = den * (xs[i] - xs[j]);
            }
        }
        acc = acc + Fp2::from_base(ys[i]) * num * Fp2::from_base(den.inv());
    }
    acc
}
