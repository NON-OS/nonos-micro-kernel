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

//! The FRI folding step. A codeword of `f` on a domain `D = shift * {omega^i}`
//! folds with a challenge into a codeword of a half-degree polynomial on the
//! squared domain. The domain may be a coset (`shift != 1`), which is what the
//! STARK needs so the low-degree test runs off the trace domain.

use super::super::field::{Fp, Fp2};
use alloc::vec::Vec;

/// Fold `evals`, the values of `f` on the size-`n` domain `{shift*omega^i}`, into
/// the values of `f_beta` on the size-`n/2` squared domain. Writing
/// `f(x) = E(x^2) + x*O(x^2)`, the even and odd parts are recovered from the pair
/// `(f(x), f(-x))` and recombined as `E + beta*O`. `inv2` is the inverse of two,
/// passed in so it is computed once by the caller.
pub fn fold_layer(evals: &[Fp], beta: Fp, shift: Fp, omega: Fp, inv2: Fp) -> Vec<Fp> {
    let half = evals.len() / 2;
    let (lo, hi) = evals.split_at(half);
    let mut out = Vec::with_capacity(half);
    // x walks the first half of the domain: shift, shift*omega, ... The point
    // paired with x is -x = shift*omega^(i + n/2), which sits at `hi[i]`.
    let mut x = shift;
    for (a, b) in lo.iter().zip(hi.iter()) {
        let even = (*a + *b) * inv2;
        let odd = (*a - *b) * inv2 * x.inv();
        out.push(even + beta * odd);
        x = x * omega;
    }
    out
}

/// The first fold, from the base-field layer 0 to an extension codeword, under an
/// extension challenge. The even and odd parts are base-field values, computed
/// exactly as in `fold_layer`; combining them with an extension `beta` lifts the
/// result into `Fp2`. Drawing the fold challenge from `Fp2` is what raises the FRI
/// soundness error from ~2^-64 to ~2^-128 (see `Nonos.Stark.Soundness`). On a base
/// `beta` this reproduces `fold_layer` embedded, so it is a faithful extension.
pub fn fold_first(evals: &[Fp], beta: Fp2, shift: Fp, omega: Fp, inv2: Fp) -> Vec<Fp2> {
    let half = evals.len() / 2;
    let (lo, hi) = evals.split_at(half);
    let mut out = Vec::with_capacity(half);
    let mut x = shift;
    for (a, b) in lo.iter().zip(hi.iter()) {
        let even = (*a + *b) * inv2;
        let odd = (*a - *b) * inv2 * x.inv();
        out.push(Fp2::from_base(even) + beta.mul_base(odd));
        x = x * omega;
    }
    out
}

/// A fold on an extension codeword under an extension challenge: every folded FRI
/// layer past the first is `Fp2`-valued, so this is the step the remaining folds
/// use. The evaluation-point arithmetic stays in the base field; only the codeword
/// values and the challenge live in the extension.
pub fn fold_ext(evals: &[Fp2], beta: Fp2, shift: Fp, omega: Fp, inv2: Fp) -> Vec<Fp2> {
    let half = evals.len() / 2;
    let (lo, hi) = evals.split_at(half);
    let mut out = Vec::with_capacity(half);
    let mut x = shift;
    for (a, b) in lo.iter().zip(hi.iter()) {
        let even = (*a + *b).mul_base(inv2);
        let odd = (*a - *b).mul_base(inv2).mul_base(x.inv());
        out.push(even + beta * odd);
        x = x * omega;
    }
    out
}
