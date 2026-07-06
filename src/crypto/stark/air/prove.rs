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

//! The STARK prover. Interpolate the trace, extend it onto an evaluation coset,
//! commit it, build the constraint composition, prove that composition is low
//! degree with FRI, and open the sampled consistency positions.

use super::super::field::Fp;
use super::super::fri::{fri_prove, root_of_unity};
use super::super::merkle::MerkleTree;
use super::super::poly::eval_lagrange;
use super::super::transcript::Transcript;
use super::constraints::{quotients, AirParams};
use super::types::{StarkProof, StarkQuery};
use alloc::vec::Vec;

/// The coset shift for the evaluation domain, a generator so the coset never
/// meets the trace subgroup.
const SHIFT: u64 = 7;

/// Prove that `trace` (length a power of two, `t[i+1] = t[i]^2`) satisfies the
/// squaring AIR with boundary `t[0]`. The evaluation domain is `2^log_blowup`
/// times the trace length.
pub fn stark_prove(trace: &[Fp], log_blowup: u32, n_queries: usize) -> StarkProof {
    let t = trace.len();
    let log_t = t.trailing_zeros();
    let log_n = log_t + log_blowup;
    let n = 1usize << log_n;
    let blowup = 1usize << log_blowup;

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    // Trace-domain points g^i and the low-degree extension of the trace onto the
    // coset D = shift * {omega^j}, by Lagrange interpolation.
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

    // Composition coefficients are drawn after the trace is committed.
    let mut transcript = Transcript::new(b"NONOS-STARK");
    transcript.absorb_digest(&trace_root);
    let alpha = transcript.challenge_fp();
    let beta = transcript.challenge_fp();

    let params = AirParams { log_t, seed: trace[0], g_last: g.pow((t - 1) as u64) };

    // The constraint composition over the coset.
    let mut comp_d: Vec<Fp> = Vec::with_capacity(n);
    let mut x = shift;
    for (j, &f_x) in trace_d.iter().enumerate() {
        let f_gx = trace_d[(j + blowup) % n];
        let (q_transition, q_boundary) = quotients(&params, x, f_x, f_gx);
        comp_d.push(alpha * q_transition + beta * q_boundary);
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
        let gx = (p + blowup) % n;
        queries.push(StarkQuery {
            comp: comp_d[p],
            comp_path: comp_tree.open(p),
            t_x: trace_d[p],
            t_x_path: trace_tree.open(p),
            t_gx: trace_d[gx],
            t_gx_path: trace_tree.open(gx),
        });
    }

    StarkProof { trace_root, fri, queries }
}
