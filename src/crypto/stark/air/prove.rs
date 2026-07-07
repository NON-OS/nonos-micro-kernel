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

//! The STARK prover, generic over any AIR. Interpolate each trace column, extend
//! it onto an evaluation coset, commit each, build the constraint composition,
//! prove that composition is low degree with FRI, and open the sampled window
//! positions across all columns.

use super::super::field::Fp;
use super::super::fri::{fri_prove, root_of_unity};
use super::super::merkle::MerkleTree;
use super::super::poly::lde;
use super::super::transcript::Transcript;
use super::composition::{compose, domain_params, num_coeffs};
use super::spec::Air;
use super::types::{StarkProof, StarkQuery};
use alloc::vec::Vec;

/// The coset shift for the evaluation domain, a generator so the coset never
/// meets the trace subgroup.
const SHIFT: u64 = 7;

/// Prove that `trace` satisfies `air`. The trace is laid out row-major:
/// `trace[row * width + col]`. The evaluation domain and the low-degree bound are
/// derived from the AIR's constraint degree.
pub fn stark_prove<A: Air>(air: &A, trace: &[Fp], n_queries: usize) -> StarkProof {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params(air);
    let n = 1usize << log_n;
    let blowup = 1usize << (log_n - log_t);
    let window_size = air.window_size();

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    // Each column is extended onto the coset by transform, then committed.
    let mut transcript = Transcript::new(b"NONOS-STARK");
    let mut trace_d: Vec<Vec<Fp>> = Vec::with_capacity(width);
    let mut trace_trees: Vec<MerkleTree> = Vec::with_capacity(width);
    let mut trace_roots: Vec<[u8; 32]> = Vec::with_capacity(width);
    for c in 0..width {
        let column: Vec<Fp> = (0..t).map(|i| trace[i * width + c]).collect();
        let column_d = lde(&column, g, shift, omega, n);
        let tree = MerkleTree::commit(&column_d);
        transcript.absorb_digest(&tree.root());
        trace_roots.push(tree.root());
        trace_trees.push(tree);
        trace_d.push(column_d);
    }

    // Composition coefficients, drawn after the whole trace is committed.
    let coeffs: Vec<Fp> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp()).collect();

    // Public periodic columns (round constants), each extended onto the coset once.
    let periodic_d: Vec<Vec<Fp>> =
        air.periodic_columns().iter().map(|col| lde(col, g, shift, omega, n)).collect();

    // The constraint composition over the coset. The window at position j gathers
    // every column at rows j, j+blowup, ... which are f(x), f(g*x), ... on D.
    let mut comp_d: Vec<Fp> = Vec::with_capacity(n);
    let mut x = shift;
    for j in 0..n {
        let mut window: Vec<Fp> = Vec::with_capacity(window_size * width);
        for k in 0..window_size {
            let idx = (j + k * blowup) % n;
            for column in &trace_d {
                window.push(column[idx]);
            }
        }
        let periodic: Vec<Fp> = periodic_d.iter().map(|pd| pd[j]).collect();
        comp_d.push(compose(air, g, x, &window, &periodic, &coeffs));
        x = x * omega;
    }

    // FRI proves the composition is low degree; its first root commits it.
    let fri = fri_prove(&comp_d, shift, fri_log_blowup, n_queries);
    let comp_tree = MerkleTree::commit(&comp_d);

    // Consistency positions, bound after the composition commitment.
    transcript.absorb_digest(&fri.roots[0]);
    let mut queries: Vec<StarkQuery> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(n);
        let mut window: Vec<Fp> = Vec::with_capacity(window_size * width);
        let mut window_paths: Vec<Vec<[u8; 32]>> = Vec::with_capacity(window_size * width);
        for k in 0..window_size {
            let idx = (p + k * blowup) % n;
            for (column, tree) in trace_d.iter().zip(trace_trees.iter()) {
                window.push(column[idx]);
                window_paths.push(tree.open(idx));
            }
        }
        queries.push(StarkQuery {
            comp: comp_d[p],
            comp_path: comp_tree.open(p),
            window,
            window_paths,
        });
    }

    StarkProof { trace_roots, fri, queries }
}
