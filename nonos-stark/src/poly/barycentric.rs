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

//! Evaluating a polynomial off its domain, given its values on a subgroup.
//!
//! The general Lagrange evaluation rebuilds every basis polynomial from scratch,
//! which is quadratic in the domain size. When the domain is the subgroup
//! `{w^i}` the basis has a closed form,
//!
//! ```text
//!   L_i(z) = (w^i / t) * (z^t - 1) / (z - w^i)
//! ```
//!
//! so the whole evaluation is one pass plus a single field inversion, shared
//! across the points by Montgomery's trick. The verifier evaluates one periodic
//! column per constraint at the out-of-domain point, so this is the difference
//! between verification growing with the square of the trace and growing with
//! the trace.

use super::super::field::{Fp, Fp2};
use alloc::vec::Vec;

/// The barycentric evaluation where it applies, and the general one where it
/// does not: a domain point, or values that do not span the whole domain. Both
/// compute the same polynomial, so the choice is only ever about cost.
pub fn eval_subgroup_or_lagrange_ext(g: Fp, h_pts: &[Fp], ys: &[Fp], z: Fp2) -> Fp2 {
    if ys.len() == h_pts.len() {
        if let Some(v) = eval_subgroup_ext(g, ys, z) {
            return v;
        }
    }
    super::lagrange::eval_lagrange_ext(h_pts, ys, z)
}

/// Evaluate at `z` the polynomial taking values `ys` on `{g^i}`, where `g` has
/// order `ys.len()`. Returns `None` when `z` lies in the domain, which leaves no
/// barycentric form; callers draw `z` outside it, and the general path covers the
/// case rather than dividing by zero.
pub fn eval_subgroup_ext(g: Fp, ys: &[Fp], z: Fp2) -> Option<Fp2> {
    let t = ys.len();
    if t == 0 {
        return Some(Fp2::ZERO);
    }

    // z - w^i for every point, and the running w^i alongside.
    let mut diffs = Vec::with_capacity(t);
    let mut pows = Vec::with_capacity(t);
    let mut w = Fp::ONE;
    for _ in 0..t {
        let d = z - Fp2::from_base(w);
        if d == Fp2::ZERO {
            return None;
        }
        diffs.push(d);
        pows.push(w);
        w = w * g;
    }

    // Montgomery's trick: one inversion for the whole batch.
    let mut prefix = Vec::with_capacity(t);
    let mut acc = Fp2::ONE;
    for d in diffs.iter() {
        prefix.push(acc);
        acc = acc * *d;
    }
    let mut running = acc.inv();

    let mut sum = Fp2::ZERO;
    for i in (0..t).rev() {
        let inv_i = running * prefix[i];
        running = running * diffs[i];
        sum = sum + Fp2::from_base(ys[i] * pows[i]) * inv_i;
    }

    // (z^t - 1) / t, then scaled by the sum.
    let zt = z.pow(t as u64) - Fp2::ONE;
    let t_inv = Fp::from_u64(t as u64).inv();
    Some(sum * zt * Fp2::from_base(t_inv))
}
