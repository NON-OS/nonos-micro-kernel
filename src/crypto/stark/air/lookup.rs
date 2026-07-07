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

//! A lookup argument in the logarithmic-derivative form (logup): prove every
//! value of a witness lies in a table. It runs a single sum
//! `S = sum_i 1/(a_i + X) - sum_j m_j/(t_j + X)` over a challenge `X`, where
//! `m_j` is the multiplicity of table entry `t_j` among the witness. The
//! rational identity `sum_i 1/(a_i + X) = sum_j m_j/(t_j + X)` holds as a
//! function of `X` exactly when the witness multiset is contained in the table
//! with those multiplicities, so a witness value outside the table leaves a
//! term the multiplicities cannot cancel and the sum is nonzero. This is the
//! primitive the monolithic recursive verifier uses to range-check the FRI
//! query indices (bit-decompose each and look up the limbs in a range table)
//! and for general table membership.
//!
//! The inverses `1/(a_i + X)` and `1/(t_j + X)` are witnessed as trace columns
//! and constrained to be true inverses; the witness, table, and multiplicities
//! are public periodic columns. The sum accumulates over the first half of the
//! trace and is pinned to zero at a checkpoint, so the last row stays free as
//! the engine requires.
//!
//! Soundness needs `X` drawn after the witness and multiplicities are
//! committed, so a prover cannot fit the multiplicities to one evaluation
//! point. In a standalone proof `X` is a public parameter and this AIR
//! establishes the arithmetic; in the recursive verifier `X` is a Fiat-Shamir
//! challenge, wired in by a copy constraint.

use super::super::field::Fp;
use super::spec::Air;
use alloc::vec::Vec;

pub struct Lookup {
    log_n: u32,
    witness: Vec<Fp>,
    table: Vec<Fp>,
    mult: Vec<Fp>,
    challenge: Fp,
}

impl Lookup {
    /// A lookup of `witness` into `table` under challenge `X`. The
    /// multiplicities are counted from the witness, so an honest instance is
    /// self-consistent and a witness value absent from the table is counted
    /// nowhere.
    pub fn new(witness: Vec<Fp>, table: Vec<Fp>, challenge: Fp) -> Lookup {
        let mult = table
            .iter()
            .map(|t| Fp::from_u64(witness.iter().filter(|w| *w == t).count() as u64))
            .collect();
        let base = witness.len().max(table.len()).max(1).next_power_of_two();
        let log_n = base.trailing_zeros();
        Lookup { log_n, witness, table, mult, challenge }
    }

    /// The multiplicities counted for the table, so a caller can commit them to
    /// a transcript before the challenge is drawn: soundness rests on the
    /// challenge following this commitment.
    pub fn multiplicities(&self) -> &[Fp] {
        &self.mult
    }

    /// The base length: the padded maximum of the witness and table lengths.
    fn base(&self) -> usize {
        1usize << self.log_n
    }

    /// The witness value, table value, and multiplicity active at row `r`, each
    /// zero past its sequence, with the two activity selectors.
    fn row(&self, r: usize) -> (Fp, Fp, Fp, Fp, Fp) {
        let a = self.witness.get(r).copied().unwrap_or(Fp::ZERO);
        let t = self.table.get(r).copied().unwrap_or(Fp::ZERO);
        let m = self.mult.get(r).copied().unwrap_or(Fp::ZERO);
        let sw = if r < self.witness.len() { Fp::ONE } else { Fp::ZERO };
        let st = if r < self.table.len() { Fp::ONE } else { Fp::ZERO };
        (a, t, m, sw, st)
    }

    /// The honest trace, the single source of truth: the two inverse columns and
    /// the running sum. `inv_a = 1/(a + X)` on witness rows, `inv_t = 1/(t + X)`
    /// on table rows, zero elsewhere; the sum is the prefix of the per-row
    /// contribution `sw * inv_a - st * m * inv_t`.
    pub fn trace(&self) -> Vec<Fp> {
        let total = 1usize << self.log_trace_len();
        let x = self.challenge;
        let mut flat = Vec::with_capacity(total * self.trace_width());
        let mut acc = Fp::ZERO;
        for r in 0..total {
            let (a, t, m, sw, st) = self.row(r);
            let inv_a = if sw == Fp::ONE { (a + x).inv() } else { Fp::ZERO };
            let inv_t = if st == Fp::ONE { (t + x).inv() } else { Fp::ZERO };
            flat.push(inv_a);
            flat.push(inv_t);
            flat.push(acc);
            acc = acc + sw * inv_a - st * (m * inv_t);
        }
        flat
    }
}

impl Air for Lookup {
    fn log_trace_len(&self) -> u32 {
        // The base length for the sum, then a checkpoint and an inert tail so
        // the final row is free.
        self.log_n + 1
    }

    fn trace_width(&self) -> usize {
        // inv_a, inv_t, and the running sum.
        3
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The worst term multiplies a selector, a multiplicity, and an inverse,
        // all degree-`trace_len` columns: three factors, degree three. Declared
        // honestly so the engine sizes the low-degree test correctly.
        3
    }

    fn num_transition(&self) -> usize {
        3
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_trace_len();
        let mut a = Vec::with_capacity(total);
        let mut t = Vec::with_capacity(total);
        let mut m = Vec::with_capacity(total);
        let mut sw = Vec::with_capacity(total);
        let mut st = Vec::with_capacity(total);
        for r in 0..total {
            let (av, tv, mv, swv, stv) = self.row(r);
            a.push(av);
            t.push(tv);
            m.push(mv);
            sw.push(swv);
            st.push(stv);
        }
        alloc::vec![a, t, m, sw, st]
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let inv_a = window[0];
        let inv_t = window[1];
        let s = window[2];
        let s_next = window[5];
        let (a, t, m, sw, st) = (periodic[0], periodic[1], periodic[2], periodic[3], periodic[4]);
        let x = self.challenge;

        // On witness rows the inverse is a true inverse; on table rows likewise.
        let inv_a_ok = sw * (inv_a * (a + x) - Fp::ONE);
        let inv_t_ok = st * (inv_t * (t + x) - Fp::ONE);
        // The sum advances by this row's contribution; selectors zero it out on
        // inactive rows, which also carries the sum unchanged past the sequences.
        let sum_step = s_next - s - sw * inv_a + st * (m * inv_t);
        alloc::vec![inv_a_ok, inv_t_ok, sum_step]
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The sum starts at zero and, after every contribution, returns to zero
        // exactly when the witness lies in the table with these multiplicities.
        alloc::vec![(2, 0, Fp::ZERO), (2, self.base(), Fp::ZERO)]
    }
}

/// A lookup of tuples: prove every witness tuple lies in a table of tuples. Each
/// tuple is compressed to a single field element under a column-folding
/// challenge `alpha`, `c(v) = sum_k alpha^k * v_k`, and the same logarithmic
/// derivative sum runs on the compressed values,
/// `sum_i 1/(c(a_i) + X) = sum_j m_j/(c(t_j) + X)`. Two tuples compress to the
/// same value only if they are equal or `alpha` hits a root of their
/// difference, so a random `alpha` makes the compression injective with
/// overwhelming probability, and a witness tuple absent from the table leaves
/// an unmatched term. This is what the monolithic verifier needs beyond scalar
/// lookup: range-checking a FRI index looks up `(limb_value, limb_position)`
/// pairs, so a valid limb in the wrong position is caught, and a folded opening
/// is an `(a, b)` pair looked up against a table of committed pairs.
///
/// The tuple columns, multiplicities, and selectors are public periodic
/// columns; the compression is formed in the constraint from the columns and
/// `alpha`, so a caller sees the same shape as the scalar lookup. Soundness
/// needs `alpha` and `X` drawn after the witness and multiplicities commit.
pub struct TupleLookup {
    log_n: u32,
    width: usize,
    witness: Vec<Vec<Fp>>,
    table: Vec<Vec<Fp>>,
    mult: Vec<Fp>,
    alpha: Fp,
    challenge: Fp,
}

impl TupleLookup {
    /// A lookup of `witness` tuples into `table` tuples, each of width `width`,
    /// under the folding challenge `alpha` and the logup challenge `X`. The
    /// multiplicities count equal tuples, element for element.
    pub fn new(
        witness: Vec<Vec<Fp>>,
        table: Vec<Vec<Fp>>,
        width: usize,
        alpha: Fp,
        challenge: Fp,
    ) -> TupleLookup {
        let mult = table
            .iter()
            .map(|t| {
                Fp::from_u64(witness.iter().filter(|w| w.as_slice() == t.as_slice()).count() as u64)
            })
            .collect();
        let base = witness.len().max(table.len()).max(1).next_power_of_two();
        let log_n = base.trailing_zeros();
        TupleLookup { log_n, width, witness, table, mult, alpha, challenge }
    }

    /// The multiplicities, so a caller can commit them before the challenges are
    /// drawn.
    pub fn multiplicities(&self) -> &[Fp] {
        &self.mult
    }

    fn base(&self) -> usize {
        1usize << self.log_n
    }

    /// Compress a tuple to one field element, `sum_k alpha^k * v_k`.
    fn compress(&self, v: &[Fp]) -> Fp {
        let mut acc = Fp::ZERO;
        let mut p = Fp::ONE;
        for c in v {
            acc = acc + p * *c;
            p = p * self.alpha;
        }
        acc
    }

    /// The compressed witness value, compressed table value, multiplicity, and
    /// the two selectors active at row `r`.
    fn row(&self, r: usize) -> (Fp, Fp, Fp, Fp, Fp) {
        let ca = self.witness.get(r).map(|w| self.compress(w)).unwrap_or(Fp::ZERO);
        let ct = self.table.get(r).map(|t| self.compress(t)).unwrap_or(Fp::ZERO);
        let m = self.mult.get(r).copied().unwrap_or(Fp::ZERO);
        let sw = if r < self.witness.len() { Fp::ONE } else { Fp::ZERO };
        let st = if r < self.table.len() { Fp::ONE } else { Fp::ZERO };
        (ca, ct, m, sw, st)
    }

    /// The honest trace, the single source of truth: the two inverse columns and
    /// the running sum on the compressed values.
    pub fn trace(&self) -> Vec<Fp> {
        let total = 1usize << self.log_trace_len();
        let x = self.challenge;
        let mut flat = Vec::with_capacity(total * self.trace_width());
        let mut acc = Fp::ZERO;
        for r in 0..total {
            let (ca, ct, m, sw, st) = self.row(r);
            let inv_a = if sw == Fp::ONE { (ca + x).inv() } else { Fp::ZERO };
            let inv_t = if st == Fp::ONE { (ct + x).inv() } else { Fp::ZERO };
            flat.push(inv_a);
            flat.push(inv_t);
            flat.push(acc);
            acc = acc + sw * inv_a - st * (m * inv_t);
        }
        flat
    }
}

impl Air for TupleLookup {
    fn log_trace_len(&self) -> u32 {
        self.log_n + 1
    }

    fn trace_width(&self) -> usize {
        3
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The compression is a linear form in the periodic tuple columns, so it
        // does not raise the degree beyond the scalar lookup: the worst term
        // still multiplies a selector, a multiplicity, and an inverse, three
        // degree-`trace_len` columns.
        3
    }

    fn num_transition(&self) -> usize {
        3
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_trace_len();
        // width witness columns, width table columns, then multiplicity and the
        // two selectors.
        let mut cols: Vec<Vec<Fp>> =
            (0..2 * self.width + 3).map(|_| Vec::with_capacity(total)).collect();
        for r in 0..total {
            for k in 0..self.width {
                cols[k].push(self.witness.get(r).map(|w| w[k]).unwrap_or(Fp::ZERO));
                cols[self.width + k].push(self.table.get(r).map(|t| t[k]).unwrap_or(Fp::ZERO));
            }
            let (_, _, m, sw, st) = self.row(r);
            cols[2 * self.width].push(m);
            cols[2 * self.width + 1].push(sw);
            cols[2 * self.width + 2].push(st);
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let inv_a = window[0];
        let inv_t = window[1];
        let s = window[2];
        let s_next = window[5];

        // Compress the tuple columns under alpha, the same linear form the trace
        // used.
        let mut ca = Fp::ZERO;
        let mut ct = Fp::ZERO;
        let mut p = Fp::ONE;
        for k in 0..self.width {
            ca = ca + p * periodic[k];
            ct = ct + p * periodic[self.width + k];
            p = p * self.alpha;
        }
        let m = periodic[2 * self.width];
        let sw = periodic[2 * self.width + 1];
        let st = periodic[2 * self.width + 2];
        let x = self.challenge;

        let inv_a_ok = sw * (inv_a * (ca + x) - Fp::ONE);
        let inv_t_ok = st * (inv_t * (ct + x) - Fp::ONE);
        let sum_step = s_next - s - sw * inv_a + st * (m * inv_t);
        alloc::vec![inv_a_ok, inv_t_ok, sum_step]
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        alloc::vec![(2, 0, Fp::ZERO), (2, self.base(), Fp::ZERO)]
    }
}
