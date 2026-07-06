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

//! The FRI folding step. A codeword of `f` over a domain `D` folds with a
//! challenge into a codeword of a half-degree polynomial over `D` squared.

use super::super::field::Fp;
use alloc::vec::Vec;

/// Fold `evals`, the values of `f` on the size-`n` domain `{omega^i}`, into the
/// values of `f_beta` on the size-`n/2` domain `{omega^(2i)}`. Writing
/// `f(x) = E(x^2) + x*O(x^2)`, the even and odd parts are recovered from the
/// pair `(f(x), f(-x))` and recombined as `E + beta*O`. `inv2` is the inverse of
/// two, passed in so it is computed once by the caller.
pub(super) fn fold_layer(evals: &[Fp], beta: Fp, omega: Fp, inv2: Fp) -> Vec<Fp> {
    let half = evals.len() / 2;
    let (lo, hi) = evals.split_at(half);
    let mut out = Vec::with_capacity(half);
    // x walks the first half of the domain: omega^0, omega^1, ... The point
    // paired with x is -x = omega^(i + n/2), which sits at `hi[i]`.
    let mut x = Fp::ONE;
    for (a, b) in lo.iter().zip(hi.iter()) {
        let even = (*a + *b) * inv2;
        let odd = (*a - *b) * inv2 * x.inv();
        out.push(even + beta * odd);
        x = x * omega;
    }
    out
}
