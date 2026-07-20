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

//! The DEEP-consistency constraint, arithmetized: the last piece a recursive
//! verifier needs, proving that a query's DEEP value is the correct combination of
//! the opened trace and composition against the out-of-domain claims. The division
//! in `verify_ext.rs`, `(v - claimed)/(x - z)`, has no native gate, so each
//! quotient is witnessed and checked by `q*(x - z) = v - claimed`; the DEEP value
//! is then the coefficient combination of those quotients. Everything lives in the
//! trace at degree one, so the checks stay degree two, and the opened values and
//! points are pinned to the proof's publics by boundaries. One query per instance
//! here; a full verifier fuses one per query.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

pub struct DeepCheck {
    pub trace_val: Fp,
    pub claimed: Fp,
    pub comp: Fp,
    pub comp_z: Fp,
    pub deep: Fp,
    pub x: Fp,
    pub z: Fp,
    pub c0: Fp,
    pub e: Fp,
}

impl DeepCheck {
    /// Columns: [trace_val, claimed, comp, comp_z, deep, x, z, c0, e, q, q_comp].
    fn transition_impl<F: Felt>(&self, window: &[F], _periodic: &[F]) -> Vec<F> {
        let (tv, cl, cp, cpz, dp) = (window[0], window[1], window[2], window[3], window[4]);
        let (x, z, c0, e, q, qc) =
            (window[5], window[6], window[7], window[8], window[9], window[10]);
        alloc::vec![
            // The trace quotient is honestly formed.
            q * (x - z) - (tv - cl),
            // The composition quotient is honestly formed.
            qc * (x - z) - (cp - cpz),
            // The DEEP value is the coefficient combination of the quotients.
            c0 * q + e * qc - dp,
        ]
    }

    /// The witness: row 0 holds the query values and the two derived quotients; the
    /// second row is inert padding (exempt from the transition).
    pub fn trace(&self) -> Vec<Fp> {
        let xz_inv = (self.x - self.z).inv();
        let q = (self.trace_val - self.claimed) * xz_inv;
        let qc = (self.comp - self.comp_z) * xz_inv;
        let row0 = [
            self.trace_val,
            self.claimed,
            self.comp,
            self.comp_z,
            self.deep,
            self.x,
            self.z,
            self.c0,
            self.e,
            q,
            qc,
        ];
        let mut trace = Vec::with_capacity(22);
        trace.extend_from_slice(&row0);
        trace.extend_from_slice(&[Fp::ZERO; 11]);
        trace
    }
}

impl Air for DeepCheck {
    fn log_trace_len(&self) -> u32 {
        1
    }

    fn trace_width(&self) -> usize {
        11
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        2
    }

    fn num_transition(&self) -> usize {
        3
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // Pin the opened values and the points to the proof's public statement; the
        // two quotient columns stay free (they are the honest witnesses).
        alloc::vec![
            (0, 0, self.trace_val),
            (1, 0, self.claimed),
            (2, 0, self.comp),
            (3, 0, self.comp_z),
            (4, 0, self.deep),
            (5, 0, self.x),
            (6, 0, self.z),
            (7, 0, self.c0),
            (8, 0, self.e),
        ]
    }
}

impl AirExt for DeepCheck {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
