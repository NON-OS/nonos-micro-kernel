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
//! single trace column; the transition reads a sliding window of consecutive
//! rows and returns the constraint values that must vanish on every trace row
//! but the last `window_size - 1`, and boundary constraints pin chosen rows to
//! public values. Any type implementing this can be proven by the same engine.

use super::super::field::Fp;
use alloc::vec::Vec;

pub trait Air {
    /// Log2 of the trace length, a power of two.
    fn log_trace_len(&self) -> u32;

    /// Number of consecutive rows the transition reads (2 reads `x, g*x`).
    fn window_size(&self) -> usize;

    /// The highest polynomial degree among the transition constraints, in the
    /// trace values. Squaring is 2, a linear recurrence 1, an `x^7` S-box 7. The
    /// engine sizes the evaluation domain and the low-degree test from this.
    fn constraint_degree(&self) -> usize;

    /// Number of transition constraints (the length of `transition`).
    fn num_transition(&self) -> usize;

    /// The transition constraint values for a window `[f(x), f(g*x), ...]`. Each
    /// must be zero on every trace row except the final `window_size - 1`.
    fn transition(&self, window: &[Fp]) -> Vec<Fp>;

    /// Boundary constraints as `(row, value)` on the trace column.
    fn boundary(&self) -> Vec<(usize, Fp)>;
}
