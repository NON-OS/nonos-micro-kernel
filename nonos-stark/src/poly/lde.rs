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

//! The low-degree extension by transform: interpolate values on the trace
//! subgroup, then evaluate the same polynomial on a larger coset. This is the
//! per-column extension the prover runs, in O(n log n).

use super::super::field::Fp;
use super::ntt::{intt, ntt};
use alloc::vec::Vec;

/// Extend `values`, the evaluations of a polynomial on the size-`values.len()`
/// subgroup `{trace_gen^i}`, onto the coset `shift * {coset_gen^i}` of size
/// `target_len`. `trace_gen` and `coset_gen` are primitive roots of unity of the
/// respective orders. The result equals evaluating the interpolating polynomial
/// at each coset point, computed by transform rather than point by point.
pub fn lde(values: &[Fp], trace_gen: Fp, shift: Fp, coset_gen: Fp, target_len: usize) -> Vec<Fp> {
    // Interpolate to coefficients over the trace subgroup.
    let mut coeffs = intt(values, trace_gen);
    // Extend the coefficient list to the target degree with zeros.
    coeffs.resize(target_len, Fp::ZERO);
    // Fold the coset shift into the coefficients: evaluating `sum c_i (shift*x)^i`
    // is evaluating `sum (c_i shift^i) x^i`, so scale then transform.
    let mut s = Fp::ONE;
    for c in coeffs.iter_mut() {
        *c = *c * s;
        s = s * shift;
    }
    ntt(&coeffs, coset_gen)
}
