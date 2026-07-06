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

//! The STARK prover, generic over any AIR. Interpolate the trace, extend it onto
//! an evaluation coset, commit it, build the constraint composition, prove that
//! composition is low degree with FRI, and open the sampled window positions.

use super::super::field::Fp;
use super::super::fri::{fri_prove, root_of_unity};
use super::super::merkle::MerkleTree;
use super::super::poly::eval_lagrange;
use super::super::transcript::Transcript;
use super::composition::{compose, num_coeffs};
use super::spec::Air;
use super::types::{StarkProof, StarkQuery};
use alloc::vec::Vec;

/// The coset shift for the evaluation domain, a generator so the coset never
/// meets the trace subgroup.
const SHIFT: u64 = 7;

/// Prove that `trace` satisfies `air` on an evaluation domain `2^log_blowup`
/// times the trace length. `trace.len()` must equal the AIR's trace length.
pub fn stark_prove<A: Air>(air: &A, trace: &[Fp], log_blowup: u32, n_queries: usize) -> StarkProof {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let log_n = log_t + log_blowup;
    let n = 1usize << log_n;
    let blowup = 1usize << log_blowup;
    let window_size = air.window_size();

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    // Trace-domain points g^i, then the low-degree extension of the trace onto
    // the coset D = shift * {omega^j} by Lagrange interpolation.
    let mut h_pts: Vec<Fp> = Vec::with_capacity(t);
    let mut hp = Fp::ONE;
    for _ in 0..t {
        h_pts.push(hp);
        hp = hp * g;
    }
    let mut trace_d: Vec<Fp> = Vec::with_capacity(n);
    let mut x = shift;
    for _ in 0..n {
        trace_d.push(eval_lagrange(&h_pts, trace, x));
        x = x * omega;
    }

    let trace_tree = MerkleTree::commit(&trace_d);
    let trace_root = trace_tree.root();

    // Composition coefficients, drawn after the trace is committed.
    let mut transcript = Transcript::new(b"NONOS-STARK");
    transcript.absorb_digest(&trace_root);
    let coeffs: Vec<Fp> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp()).collect();

    // The constraint composition over the coset. The window at position j reads
    // the trace j, j+blowup, ... which are exactly f(x), f(g*x), ... on D.
    let mut comp_d: Vec<Fp> = Vec::with_capacity(n);
    let mut x = shift;
    for j in 0..n {
        let mut window: Vec<Fp> = Vec::with_capacity(window_size);
        for k in 0..window_size {
            window.push(trace_d[(j + k * blowup) % n]);
        }
        comp_d.push(compose(air, g, x, &window, &coeffs));
        x = x * omega;
    }

    // FRI proves the composition is low degree; its first root commits it.
    let fri = fri_prove(&comp_d, shift, log_blowup, n_queries);
    let comp_tree = MerkleTree::commit(&comp_d);

    // Consistency positions, bound after the composition commitment.
    transcript.absorb_digest(&fri.roots[0]);
    let mut queries: Vec<StarkQuery> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(n);
        let mut window: Vec<Fp> = Vec::with_capacity(window_size);
        let mut window_paths: Vec<Vec<[u8; 32]>> = Vec::with_capacity(window_size);
        for k in 0..window_size {
            let idx = (p + k * blowup) % n;
            window.push(trace_d[idx]);
            window_paths.push(trace_tree.open(idx));
        }
        queries.push(StarkQuery {
            comp: comp_d[p],
            comp_path: comp_tree.open(p),
            window,
            window_paths,
        });
    }

    StarkProof { trace_root, fri, queries }
}
