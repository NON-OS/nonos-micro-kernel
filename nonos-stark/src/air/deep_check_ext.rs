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
    witness_terms: bool,
    // Production form only: the out-of-domain point (one value, bound to the
    // transcript) and the structural g^k schedule, so each term's point is derived
    // as z * g^k rather than trusted as a free input.
    z_ood: Fp2,
    gk: Vec<Fp>,
}

impl DeepCheckExt {
    /// Build the check for `terms` at evaluation point `x`, whose batched
    /// combination must equal `deep`. The per-term public data rides the periodic
    /// columns and the final sum is pinned to `deep`: instance-specific structure.
    pub fn new(terms: Vec<DeepTerm>, x: Fp2, deep: Fp2) -> DeepCheckExt {
        let log_rows = (terms.len() + 1).next_power_of_two().trailing_zeros();
        DeepCheckExt {
            log_rows,
            terms,
            x,
            deep,
            witness_terms: false,
            z_ood: Fp2::ZERO,
            gk: Vec::new(),
        }
    }

    /// The production form: the per-term data (val, claim, point, coeff) and the
    /// evaluation point x ride the trace, so the AIR is instance-independent (the
    /// term and composition selectors are the only periodic columns, acc-starts-zero
    /// the only boundary). x is constrained constant so every term shares one point,
    /// and the terms and the final DEEP value are bound by the assembly's grand
    /// product to their sources.
    pub fn new_witness(terms: Vec<DeepTerm>, x: Fp2, deep: Fp2) -> DeepCheckExt {
        let log_rows = (terms.len() + 1).next_power_of_two().trailing_zeros();
        // The out-of-domain point is the point of the first term (window row zero),
        // and each term's structural g^k factor is its point divided by z. The
        // schedule is instance-independent (g^k for the inner trace generator); the
        // division just recovers it from the terms the caller already holds.
        let z_ood = terms.first().map(|t| t.point).unwrap_or(Fp2::ONE);
        let zinv = z_ood.inv();
        let gk: Vec<Fp> = terms.iter().map(|t| (t.point * zinv).c0).collect();
        DeepCheckExt { log_rows, terms, x, deep, witness_terms: true, z_ood, gk }
    }

    /// The witness: per active row the quotient and the running sum through that
    /// term; the row after the last term carries the final sum, the rest padded.
    pub fn trace(&self) -> Vec<Fp> {
        let rows = 1usize << self.log_rows;
        let w = self.trace_width();
        let mut trace = alloc::vec![Fp::ZERO; rows * w];
        // The composition claim, held on every row so it is a wireable trace cell.
        let comp_z = self.terms.last().map(|t| t.claim).unwrap_or(Fp2::ZERO);
        for r in 0..rows {
            trace[r * w + 4] = comp_z.c0;
            trace[r * w + 5] = comp_z.c1;
            // x and the out-of-domain point z are constant on every row, so both can
            // be constrained equal across terms and bound once by the assembly.
            if self.witness_terms {
                trace[r * w + 10] = self.z_ood.c0;
                trace[r * w + 11] = self.z_ood.c1;
                trace[r * w + 14] = self.x.c0;
                trace[r * w + 15] = self.x.c1;
            }
        }
        let mut acc = Fp2::ZERO;
        for (i, term) in self.terms.iter().enumerate() {
            let q = (term.val - term.claim) * (self.x - term.point).inv();
            let base = i * w;
            trace[base] = q.c0;
            trace[base + 1] = q.c1;
            trace[base + 2] = acc.c0;
            trace[base + 3] = acc.c1;
            if self.witness_terms {
                trace[base + 6] = term.val.c0;
                trace[base + 7] = term.val.c1;
                trace[base + 8] = term.claim.c0;
                trace[base + 9] = term.claim.c1;
                trace[base + 12] = term.coeff.c0;
                trace[base + 13] = term.coeff.c1;
            }
            acc = acc + term.coeff * q;
        }
        let base = self.terms.len() * w;
        trace[base + 2] = acc.c0;
        trace[base + 3] = acc.c1;
        trace
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let stride = self.trace_width();
        let (q0, q1) = (window[0], window[1]);
        let (acc0, acc1) = (window[2], window[3]);
        let (cz0, cz1) = (window[4], window[5]);
        let (nacc0, nacc1) = (window[stride + 2], window[stride + 3]);
        // In the per-proof form the per-term data and x ride the periodic columns and
        // the point is given. In the production form val/claim/coeff/x/z ride the
        // trace, the selectors and the structural g^k schedule ride the periodic
        // columns, and the point is derived as z * g^k so it is never a free input.
        let (val0, val1, clm0, clm1, pt0, pt1, cf0, cf1, x0, x1, sel, comp_sel) =
            if self.witness_terms {
                let (z0, z1) = (window[10], window[11]);
                let gk = periodic[2];
                (
                    window[6],
                    window[7],
                    window[8],
                    window[9],
                    z0 * gk,
                    z1 * gk,
                    window[12],
                    window[13],
                    window[14],
                    window[15],
                    periodic[0],
                    periodic[1],
                )
            } else {
                (
                    periodic[0],
                    periodic[1],
                    periodic[2],
                    periodic[3],
                    periodic[4],
                    periodic[5],
                    periodic[6],
                    periodic[7],
                    periodic[8],
                    periodic[9],
                    periodic[10],
                    periodic[11],
                )
            };
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

        let mut out = alloc::vec![
            sel * (qd0 - n0),
            sel * (qd1 - n1),
            nacc0 - acc0 - sel * cq0,
            nacc1 - acc1 - sel * cq1,
            // The wired composition cell equals the composition term's claim.
            comp_sel * (cz0 - clm0),
            comp_sel * (cz1 - clm1),
        ];
        // The witnessed evaluation point x and out-of-domain point z are constant, so
        // every term shares one x and one z (from which each point is derived).
        if self.witness_terms {
            out.push(window[stride + 14] - window[14]);
            out.push(window[stride + 15] - window[15]);
            out.push(window[stride + 10] - window[10]);
            out.push(window[stride + 11] - window[11]);
        }
        out
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
        if self.witness_terms {
            16
        } else {
            6
        }
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // Per-proof: quotient times point, coefficient times quotient, then the
        // selector: three factors. Production: the point is z * g^k, so the quotient
        // constraint gains one factor (q * z * g^k under the selector): four.
        if self.witness_terms {
            4
        } else {
            3
        }
    }

    fn num_transition(&self) -> usize {
        // Production adds the x-constant and z-constant constraints (two each).
        if self.witness_terms {
            10
        } else {
            6
        }
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let rows = 1usize << self.log_rows;
        // Production form: the term and composition selectors plus the structural
        // g^k schedule (one per term row), all instance-independent.
        if self.witness_terms {
            let mut sel = alloc::vec![Fp::ZERO; rows];
            let mut comp_sel = alloc::vec![Fp::ZERO; rows];
            let mut gk = alloc::vec![Fp::ZERO; rows];
            for (i, item) in sel.iter_mut().take(self.terms.len()).enumerate() {
                *item = Fp::ONE;
                gk[i] = self.gk[i];
            }
            if !self.terms.is_empty() {
                comp_sel[self.terms.len() - 1] = Fp::ONE;
            }
            return alloc::vec![sel, comp_sel, gk];
        }
        let mut cols: Vec<Vec<Fp>> = (0..12).map(|_| alloc::vec![Fp::ZERO; rows]).collect();
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
        // The composition term (the last one) is where the wired cell must equal
        // the claim.
        if !self.terms.is_empty() {
            cols[11][self.terms.len() - 1] = Fp::ONE;
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The running sum starts at zero. In the per-proof form the final sum is
        // pinned to the query's DEEP value; in the production form that final sum is
        // a witness cell bound by the assembly's grand product, so only the
        // structural acc-starts-zero boundary remains.
        if self.witness_terms {
            return alloc::vec![(2, 0, Fp::ZERO), (3, 0, Fp::ZERO)];
        }
        alloc::vec![
            (2, 0, Fp::ZERO),
            (3, 0, Fp::ZERO),
            (2, self.terms.len(), self.deep.c0),
            (3, self.terms.len(), self.deep.c1),
        ]
    }
}
