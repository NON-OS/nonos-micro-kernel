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

//! The AIR under proof: a squaring chain. The trace is a single column with
//! `t[0] = seed` and `t[i+1] = t[i]^2`. Two constraints capture it: a transition
//! `f(g*x) - f(x)^2 = 0` on every trace row but the last, and a boundary
//! `f(1) - seed = 0` on the first. Both are turned into quotients here, in one
//! function shared by prover and verifier so the algebra cannot drift.

use super::super::field::Fp;

pub(super) struct AirParams {
    /// Log2 of the trace length.
    pub log_t: u32,
    /// The public boundary value at row zero.
    pub seed: Fp,
    /// The last trace-domain point `g^(T-1)`, excluded from the transition.
    pub g_last: Fp,
}

/// The transition and boundary quotients at a coset point `x`, given the trace
/// value there (`f_x`) and one row ahead (`f_gx`). A quotient is a polynomial
/// exactly when its constraint holds across the trace domain, which is what the
/// low-degree test then checks. `x` lies off the trace domain, so the vanishing
/// polynomial and the boundary divisor are both nonzero and invertible.
pub(super) fn quotients(params: &AirParams, x: Fp, f_x: Fp, f_gx: Fp) -> (Fp, Fp) {
    let t = 1u64 << params.log_t;
    // Z_H(x) = x^T - 1 vanishes exactly on the trace domain.
    let z_h = x.pow(t) - Fp::ONE;
    // Transition holds on all rows but the last, so multiply the numerator by the
    // excluded point before dividing by the full vanishing polynomial.
    let transition = (f_gx - f_x * f_x) * (x - params.g_last);
    let q_transition = transition * z_h.inv();
    // Boundary vanishes at the first row, domain point one.
    let q_boundary = (f_x - params.seed) * (x - Fp::ONE).inv();
    (q_transition, q_boundary)
}
