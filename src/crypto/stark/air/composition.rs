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

//! Combining an AIR's constraints into one composition value at a coset point.
//! Each transition constraint becomes a quotient by the trace-domain vanishing
//! polynomial (with the exempt final rows multiplied back in), each boundary a
//! quotient by its single vanishing point, and all are folded under transcript
//! coefficients. Prover and verifier both call this, so the algebra is identical
//! on the two sides by construction.

use super::super::field::Fp;
use super::spec::Air;

/// Total number of random coefficients an AIR's composition consumes.
pub(super) fn num_coeffs<A: Air>(air: &A) -> usize {
    air.num_transition() + air.boundary().len()
}

/// The composition value at coset point `x`, given the trace window
/// `[f(x), f(g*x), ...]` and the transcript coefficients. `g` is the trace-domain
/// generator. `x` lies off the trace domain, so every divisor is invertible.
pub(super) fn compose<A: Air>(air: &A, g: Fp, x: Fp, window: &[Fp], coeffs: &[Fp]) -> Fp {
    let t = 1u64 << air.log_trace_len();
    let z_h_inv = (x.pow(t) - Fp::ONE).inv();

    // The final `window_size - 1` rows have no successor, so exempt them by
    // multiplying the transition numerator by their vanishing points.
    let mut exempt = Fp::ONE;
    for k in 1..air.window_size() {
        exempt = exempt * (x - g.pow(t - k as u64));
    }

    let mut acc = Fp::ZERO;
    let transition = air.transition(window);
    for (value, coeff) in transition.iter().zip(coeffs.iter()) {
        acc = acc + *coeff * (*value * exempt * z_h_inv);
    }

    let boundary_coeffs = &coeffs[transition.len()..];
    for ((row, expected), coeff) in air.boundary().iter().zip(boundary_coeffs.iter()) {
        let quotient = (window[0] - *expected) * (x - g.pow(*row as u64)).inv();
        acc = acc + *coeff * quotient;
    }

    acc
}
