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

//! A copy constraint over a value column, the Plonk permutation argument built
//! on the grand product. A public wiring permutation `sigma` declares which
//! positions must hold equal values; the identity
//! `prod (w_i + b*i + g) = prod (w_i + b*sigma_i + g)` holds exactly when the
//! values are constant on the cycles of `sigma`, so it proves the wiring. This
//! is what a monolithic recursive verifier uses to bind a value produced in one
//! region of a trace to where it is consumed in another, without laying them out
//! adjacently.

use super::super::field::Fp;
use super::spec::Air;
use alloc::vec::Vec;

pub struct CopyConstraint {
    log_n: u32,
    /// The wired values; equal across every cycle of `sigma`.
    values: Vec<Fp>,
    /// The wiring permutation of positions.
    sigma: Vec<usize>,
    beta: Fp,
    gamma: Fp,
}

impl CopyConstraint {
    pub fn new(values: Vec<Fp>, sigma: Vec<usize>, beta: Fp, gamma: Fp) -> CopyConstraint {
        let log_n = values.len().trailing_zeros();
        CopyConstraint { log_n, values, sigma, beta, gamma }
    }

    fn len(&self) -> usize {
        1usize << self.log_n
    }

    /// The witness trace: the value column and the running product that closes to
    /// one exactly when the wiring holds.
    pub fn trace(&self) -> Vec<Fp> {
        let n = self.len();
        let total = 1usize << self.log_trace_len();
        let (b, g) = (self.beta, self.gamma);
        let mut z = Fp::ONE;
        let mut trace = Vec::with_capacity(total * 2);
        for r in 0..total {
            let w = if r < n { self.values[r] } else { Fp::ZERO };
            trace.push(w);
            trace.push(z);
            if r < n {
                let num = w + b * Fp::from_u64(r as u64) + g;
                let den = w + b * Fp::from_u64(self.sigma[r] as u64) + g;
                z = z * num * den.inv();
            }
        }
        trace
    }
}

impl Air for CopyConstraint {
    fn log_trace_len(&self) -> u32 {
        self.log_n + 1
    }

    fn trace_width(&self) -> usize {
        2
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        3
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let n = self.len();
        let total = 1usize << self.log_trace_len();
        let mut id = Vec::with_capacity(total);
        let mut sig = Vec::with_capacity(total);
        let mut sel = Vec::with_capacity(total);
        for r in 0..total {
            let active = r < n;
            id.push(if active { Fp::from_u64(r as u64) } else { Fp::ZERO });
            sig.push(if active { Fp::from_u64(self.sigma[r] as u64) } else { Fp::ZERO });
            sel.push(if active { Fp::ONE } else { Fp::ZERO });
        }
        alloc::vec![id, sig, sel]
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let w = window[0];
        let z = window[1];
        let z_next = window[3];
        let id = periodic[0];
        let sig = periodic[1];
        let sel = periodic[2];
        let (b, g) = (self.beta, self.gamma);

        // z_next * (w + b*sigma + g) = z * (w + b*id + g) in the product region;
        // z carried otherwise.
        let product = z_next * (w + b * sig + g) - z * (w + b * id + g);
        let carry = z_next - z;
        alloc::vec![sel * product + (Fp::ONE - sel) * carry]
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // Column 1 is the running product: one at the start, and one at the
        // checkpoint exactly when the wiring holds.
        alloc::vec![(1, 0, Fp::ONE), (1, self.len(), Fp::ONE)]
    }
}
