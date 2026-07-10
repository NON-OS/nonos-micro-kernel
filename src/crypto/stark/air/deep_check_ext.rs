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

//! The multi-term DEEP-consistency gadget over the extension: the money-grade
//! counterpart of `DeepCheck`. A real money-grade query proves one DEEP value is the
//! batched combination of every opened trace column against its out-of-domain claim,
//! plus the composition against its claim, each divided by `x - point`. This runs
//! the combination one term per row: it witnesses each quotient `q = (val - claim) /
//! (x - point)`, checks `q * (x - point) = val - claim`, and accumulates `coeff * q`
//! into a running sum whose final value is pinned to the query's DEEP value. Every
//! value is `Fp2`, laid out as base column pairs, its multiplication expanded with
//! the `X^2 = 7` cross terms so the transition is generic over the field. The
//! per-term public data (val, claim, point, coeff, x) rides the periodic columns;
//! the quotient and the running sum are the witness.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// The extension non-residue, so `(p + q X)(r + s X) = (pr + W qs) + (ps + qr) X`.
const W: u64 = 7;

/// One DEEP term: an opened value, its out-of-domain claim, the point it is claimed
/// at, and its batching coefficient.
pub struct DeepTerm {
    pub val: Fp2,
    pub claim: Fp2,
    pub point: Fp2,
    pub coeff: Fp2,
}

pub struct DeepCheckExt {
    log_rows: u32,
    terms: Vec<DeepTerm>,
    x: Fp2,
    deep: Fp2,
}

impl DeepCheckExt {
    /// Build the check for `terms` at evaluation point `x`, whose batched
    /// combination must equal `deep`.
    pub fn new(terms: Vec<DeepTerm>, x: Fp2, deep: Fp2) -> DeepCheckExt {
        let log_rows = (terms.len() + 1).next_power_of_two().trailing_zeros();
        DeepCheckExt { log_rows, terms, x, deep }
    }

    /// The witness: per active row the quotient and the running sum through that
    /// term; the row after the last term carries the final sum, the rest padded.
    pub fn trace(&self) -> Vec<Fp> {
        let rows = 1usize << self.log_rows;
        let mut trace = alloc::vec![Fp::ZERO; rows * 4];
        let mut acc = Fp2::ZERO;
        for (i, term) in self.terms.iter().enumerate() {
            let q = (term.val - term.claim) * (self.x - term.point).inv();
            let base = i * 4;
            trace[base] = q.c0;
            trace[base + 1] = q.c1;
            trace[base + 2] = acc.c0;
            trace[base + 3] = acc.c1;
            acc = acc + term.coeff * q;
        }
        let base = self.terms.len() * 4;
        trace[base + 2] = acc.c0;
        trace[base + 3] = acc.c1;
        trace
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let (q0, q1) = (window[0], window[1]);
        let (acc0, acc1) = (window[2], window[3]);
        let (nacc0, nacc1) = (window[6], window[7]);
        let (val0, val1) = (periodic[0], periodic[1]);
        let (clm0, clm1) = (periodic[2], periodic[3]);
        let (pt0, pt1) = (periodic[4], periodic[5]);
        let (cf0, cf1) = (periodic[6], periodic[7]);
        let (x0, x1) = (periodic[8], periodic[9]);
        let sel = periodic[10];
        let w = F::from_base(Fp::from_u64(W));

        // q * (x - point) in Fp2.
        let (d0, d1) = (x0 - pt0, x1 - pt1);
        let qd0 = q0 * d0 + w * q1 * d1;
        let qd1 = q0 * d1 + q1 * d0;
        // val - claim.
        let (n0, n1) = (val0 - clm0, val1 - clm1);

        // coeff * q in Fp2.
        let cq0 = cf0 * q0 + w * cf1 * q1;
        let cq1 = cf0 * q1 + cf1 * q0;

        alloc::vec![
            sel * (qd0 - n0),
            sel * (qd1 - n1),
            nacc0 - acc0 - sel * cq0,
            nacc1 - acc1 - sel * cq1,
        ]
    }
}

impl AirExt for DeepCheckExt {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for DeepCheckExt {
    fn log_trace_len(&self) -> u32 {
        self.log_rows
    }

    fn trace_width(&self) -> usize {
        4
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The quotient times the public point, and the coefficient times the
        // quotient, then the selector: three interpolated factors.
        3
    }

    fn num_transition(&self) -> usize {
        4
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let rows = 1usize << self.log_rows;
        let mut cols: Vec<Vec<Fp>> = (0..11).map(|_| alloc::vec![Fp::ZERO; rows]).collect();
        for (i, term) in self.terms.iter().enumerate() {
            cols[0][i] = term.val.c0;
            cols[1][i] = term.val.c1;
            cols[2][i] = term.claim.c0;
            cols[3][i] = term.claim.c1;
            cols[4][i] = term.point.c0;
            cols[5][i] = term.point.c1;
            cols[6][i] = term.coeff.c0;
            cols[7][i] = term.coeff.c1;
            cols[10][i] = Fp::ONE;
        }
        // x is constant on every row.
        cols[8].iter_mut().for_each(|c| *c = self.x.c0);
        cols[9].iter_mut().for_each(|c| *c = self.x.c1);
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The running sum starts at zero and, after the last term, equals the
        // query's DEEP value.
        alloc::vec![
            (2, 0, Fp::ZERO),
            (3, 0, Fp::ZERO),
            (2, self.terms.len(), self.deep.c0),
            (3, self.terms.len(), self.deep.c1),
        ]
    }
}
