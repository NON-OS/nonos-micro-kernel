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

//! The algebraic intermediate representation a STARK proves. A computation is a
//! trace of `trace_width` columns; the transition reads a sliding window of
//! consecutive rows and returns the constraint values that must vanish on every
//! trace row but the last `window_size - 1`, and boundary constraints pin a
//! chosen `(column, row)` to a public value. A single column proves a chain; a
//! wider trace proves a permutation over a multi-element state, which is what a
//! hash round is. Any type implementing this is proven by the same engine.

use super::super::field::{Fp, Fp2};
use alloc::vec::Vec;

pub trait Air {
    /// Log2 of the trace length, a power of two.
    fn log_trace_len(&self) -> u32;

    /// Number of trace columns, the width of the state.
    fn trace_width(&self) -> usize;

    /// Number of consecutive rows the transition reads (2 reads a row and its
    /// successor).
    fn window_size(&self) -> usize;

    /// The highest polynomial degree among the transition constraints, in the
    /// trace values. Squaring is 2, a linear recurrence 1, an `x^7` S-box 7. The
    /// engine sizes the evaluation domain and the low-degree test from this.
    fn constraint_degree(&self) -> usize;

    /// Number of transition constraints (the length of `transition`).
    fn num_transition(&self) -> usize;

    /// Public per-row values, one vector of length `trace_len` per column. They
    /// are interpolated over the trace domain and the engine hands the transition
    /// their value at the current point. This is how round constants, which
    /// differ every row, enter the constraints. Default: none.
    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        Vec::new()
    }

    /// The transition constraint values. `window` is laid out row-major:
    /// `window[k * trace_width + col]` is column `col` of the k-th row in the
    /// window. `periodic` holds the periodic columns evaluated at the current
    /// point, in declaration order. Each returned value must be zero on every
    /// trace row except the final `window_size - 1`.
    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp>;

    /// Boundary constraints as `(column, row, value)`.
    fn boundary(&self) -> Vec<(usize, usize, Fp)>;
}

/// An AIR whose transition also evaluates over the extension field. A money-grade
/// STARK samples its out-of-domain point at `z in Fp2`, so the constraints must be
/// evaluable there. An AIR implements this by writing its transition once over the
/// `Felt` abstraction and delegating both `transition` and `transition_ext` to it,
/// which keeps the trait object-safe. Only AIRs proven by the money-grade engine
/// (`stark_prove_ext`) need it.
// `Sync` lets the prover evaluate the composition and DEEP polynomials across
// cores under the `parallel` feature; every AIR is plain data, so the bound is
// satisfied automatically and the kernel build is unaffected.
pub trait AirExt: Air + Sync {
    /// The transition constraints over the extension field, layout as `transition`.
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2>;
}
