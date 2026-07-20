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

use super::super::field::{Fp, Fp2};
use super::spec::{Air, AirExt};

/// Total number of random coefficients an AIR's composition consumes.
pub(super) fn num_coeffs<A: Air>(air: &A) -> usize {
    air.num_transition() + air.boundary().len()
}

/// The evaluation-domain sizing derived from the AIR: `(log_n, fri_log_blowup)`.
///
/// The composition degree is below `constraint_degree * trace_len`, so the FRI
/// degree bound `B` is that rounded up to a power of two, and the evaluation
/// domain is `2 * B` (a rate of one half for the low-degree test). Both prover
/// and verifier call this, so they agree on the domain without passing sizes.
pub(super) fn domain_params<A: Air>(air: &A) -> (u32, u32) {
    domain_params_blown(air, 0)
}

/// The evaluation domain and FRI blowup with `extra_blowup_bits` of low-degree
/// headroom folded into the FRI rate. Zero reproduces the minimal 2 * B domain
/// (rate one half). Each added bit doubles the domain and halves the rate, so a
/// single FRI query catches a non-low-degree codeword with proportionally higher
/// probability: this is the query-count-versus-soundness lever a deployment vector
/// sets to reach 128-bit security without paying for a hundred queries. The extra
/// cost is a larger LDE, all prover side.
pub(super) fn domain_params_blown<A: Air>(air: &A, extra_blowup_bits: u32) -> (u32, u32) {
    let t = 1usize << air.log_trace_len();
    let degree = air.constraint_degree().max(1);
    let bound = (degree * t).next_power_of_two();
    let n = (2 * bound) << extra_blowup_bits;
    let log_n = n.trailing_zeros();
    let fri_log_blowup = log_n - bound.trailing_zeros();
    (log_n, fri_log_blowup)
}

/// The composition value at coset point `x`, given the trace window
/// `[f(x), f(g*x), ...]`, the periodic columns evaluated at `x`, and the
/// transcript coefficients. `g` is the trace-domain generator. `x` lies off the
/// trace domain, so every divisor is invertible.
pub fn compose<A: Air>(air: &A, g: Fp, x: Fp, window: &[Fp], periodic: &[Fp], coeffs: &[Fp]) -> Fp {
    let t = 1u64 << air.log_trace_len();
    let z_h_inv = (x.pow(t) - Fp::ONE).inv();

    // The final `window_size - 1` rows have no successor, so exempt them by
    // multiplying the transition numerator by their vanishing points.
    let mut exempt = Fp::ONE;
    for k in 1..air.window_size() {
        exempt = exempt * (x - g.pow(t - k as u64));
    }

    let mut acc = Fp::ZERO;
    let transition = air.transition(window, periodic);
    for (value, coeff) in transition.iter().zip(coeffs.iter()) {
        acc = acc + *coeff * (*value * exempt * z_h_inv);
    }

    // Boundary quotients read column `col` at window offset zero, `window[col]`.
    let boundary_coeffs = &coeffs[transition.len()..];
    for ((col, row, expected), coeff) in air.boundary().iter().zip(boundary_coeffs.iter()) {
        let quotient = (window[*col] - *expected) * (x - g.pow(*row as u64)).inv();
        acc = acc + *coeff * quotient;
    }

    acc
}

/// The composition value at an extension point `z in Fp2`, the out-of-domain
/// evaluation a money-grade STARK samples. Same algebra as `compose`, over the
/// extension: base-field constants (the trace generator powers, boundary values)
/// enter through `Fp2::from_base`, and the batching coefficients are drawn from the
/// extension for the same soundness. On a base-embedded point and base-embedded
/// inputs it agrees with `compose` embedded, by construction.
pub fn compose_ext<A: AirExt>(
    air: &A,
    g: Fp,
    z: Fp2,
    window: &[Fp2],
    periodic: &[Fp2],
    coeffs: &[Fp2],
) -> Fp2 {
    let t = 1u64 << air.log_trace_len();
    let z_h_inv = (z.pow(t) - Fp2::ONE).inv();

    let mut exempt = Fp2::ONE;
    for k in 1..air.window_size() {
        exempt = exempt * (z - Fp2::from_base(g.pow(t - k as u64)));
    }

    let mut acc = Fp2::ZERO;
    let transition = air.transition_ext(window, periodic);
    for (value, coeff) in transition.iter().zip(coeffs.iter()) {
        acc = acc + *coeff * (*value * exempt * z_h_inv);
    }

    let boundary_coeffs = &coeffs[transition.len()..];
    for ((col, row, expected), coeff) in air.boundary().iter().zip(boundary_coeffs.iter()) {
        let quotient = (window[*col] - Fp2::from_base(*expected))
            * (z - Fp2::from_base(g.pow(*row as u64))).inv();
        acc = acc + *coeff * quotient;
    }

    acc
}
