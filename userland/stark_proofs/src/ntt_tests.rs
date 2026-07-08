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

use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poly::{eval, eval_lagrange, intt, lde, ntt};

extern crate alloc;
use alloc::vec::Vec;

// The transform is the prover's scaling path. It must agree exactly with the
// direct methods it replaces: evaluation on the subgroup, and the Lagrange
// low-degree extension onto the coset. These checks pin that equality.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn random(n: usize, seed: u64) -> Vec<Fp> {
    let mut s = seed | 1;
    (0..n).map(|_| Fp::from_u64(xorshift(&mut s))).collect()
}

#[test]
fn ntt_matches_pointwise_evaluation() {
    for log_n in [1u32, 2, 3, 5, 8] {
        let n = 1usize << log_n;
        let omega = root_of_unity(log_n);
        let coeffs = random(n, 0x1234 + log_n as u64);
        let evals = ntt(&coeffs, omega);
        let mut x = Fp::ONE;
        for (j, &e) in evals.iter().enumerate() {
            assert_eq!(e, eval(&coeffs, x), "ntt disagrees with evaluation at {j}");
            x = x * omega;
        }
    }
}

#[test]
fn intt_inverts_ntt() {
    for log_n in [1u32, 3, 5, 8] {
        let n = 1usize << log_n;
        let omega = root_of_unity(log_n);
        let coeffs = random(n, 0xabcd + log_n as u64);
        assert_eq!(intt(&ntt(&coeffs, omega), omega), coeffs, "intt did not invert ntt");
    }
}

#[test]
fn lde_matches_the_lagrange_extension() {
    // Extend a size-16 trace onto a size-64 coset and compare against the direct
    // Lagrange evaluation at every coset point.
    let (log_t, log_n) = (4u32, 6u32);
    let (t, n) = (1usize << log_t, 1usize << log_n);
    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(7);

    let values = random(t, 0x9e37);
    let fast = lde(&values, g, shift, omega, n);

    let h_pts: Vec<Fp> = {
        let mut v = Vec::with_capacity(t);
        let mut p = Fp::ONE;
        for _ in 0..t {
            v.push(p);
            p = p * g;
        }
        v
    };
    let mut x = shift;
    for (j, &f) in fast.iter().enumerate() {
        assert_eq!(f, eval_lagrange(&h_pts, &values, x), "lde disagrees with lagrange at {j}");
        x = x * omega;
    }
}
