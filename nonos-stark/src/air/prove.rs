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

//! The DEEP STARK prover, generic over any AIR. Commit the extended trace and
//! the constraint composition, evaluate the trace at an out-of-domain point
//! drawn from the transcript, and prove with FRI that the DEEP quotients, which
//! tie every committed value to those out-of-domain evaluations, are one low
//! degree polynomial. The out-of-domain binding is what lets each query open a
//! single row instead of a whole constraint window.

use super::super::field::Fp;
use super::super::fri::{fri_prove, root_of_unity};
use super::super::merkle::MerkleTree;
use super::super::poly::{eval, intt, lde};
use super::super::transcript::Transcript;
use super::composition::{compose, domain_params, num_coeffs};
use super::spec::Air;
use super::types::{StarkProof, StarkQuery};
use alloc::vec::Vec;

/// The coset shift for the evaluation domain, a generator so the coset never
/// meets the trace subgroup.
const SHIFT: u64 = 7;

/// Draw the out-of-domain point: a challenge that lies neither on the
/// evaluation coset nor on the trace domain, so every DEEP denominator and
/// every periodic-column denominator is invertible. Both sides run this
/// identically, so they agree on the point.
pub(super) fn draw_ood_point(transcript: &mut Transcript, shift: Fp, n: usize, t: usize) -> Fp {
    let shift_n = shift.pow(n as u64);
    let mut z = transcript.challenge_fp();
    while z.pow(n as u64) == shift_n || z.pow(t as u64) == Fp::ONE {
        z = transcript.challenge_fp();
    }
    z
}

/// Prove that `trace` satisfies `air`. The trace is laid out row-major:
/// `trace[row * width + col]`. The evaluation domain and the low-degree bound
/// are derived from the AIR's constraint degree.
pub fn stark_prove<A: Air>(air: &A, trace: &[Fp], n_queries: usize) -> StarkProof {
    stark_prove_bound(air, trace, n_queries, &[])
}

/// Prove `trace` bound to `context`. The context is absorbed into the transcript
/// before anything else, so the challenges depend on it and the proof only verifies
/// under the same context. An empty context reduces to `stark_prove`.
pub fn stark_prove_bound<A: Air>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    context: &[u8],
) -> StarkProof {
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

    // Each column is extended onto the coset by transform and committed; its
    // coefficients are kept for the out-of-domain evaluations.
    let mut transcript = Transcript::new(b"NONOS-STARK");
    if !context.is_empty() {
        transcript.absorb_digest(&crate::hash::keccak256(context));
    }
    let mut trace_d: Vec<Vec<Fp>> = Vec::with_capacity(width);
    let mut trace_coeffs: Vec<Vec<Fp>> = Vec::with_capacity(width);
    let mut trace_trees: Vec<MerkleTree> = Vec::with_capacity(width);
    let mut trace_roots: Vec<[u8; 32]> = Vec::with_capacity(width);
    for c in 0..width {
        let column: Vec<Fp> = (0..t).map(|i| trace[i * width + c]).collect();
        let column_d = lde(&column, g, shift, omega, n);
        let tree = MerkleTree::commit(&column_d);
        transcript.absorb_digest(&tree.root());
        trace_roots.push(tree.root());
        trace_coeffs.push(intt(&column, g));
        trace_trees.push(tree);
        trace_d.push(column_d);
    }

    // Composition coefficients, drawn after the whole trace is committed.
    let coeffs: Vec<Fp> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp()).collect();

    // Public periodic columns (round constants), each extended onto the coset.
    let periodic_cols = air.periodic_columns();
    let periodic_d: Vec<Vec<Fp>> =
        periodic_cols.iter().map(|col| lde(col, g, shift, omega, n)).collect();

    // The constraint composition over the coset, committed on its own.
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
    let comp_tree = MerkleTree::commit(&comp_d);
    transcript.absorb_digest(&comp_tree.root());

    // The out-of-domain point and the trace frame there: every column at
    // g^k * z for each window row k.
    let z = draw_ood_point(&mut transcript, shift, n, t);
    let mut ood_frame: Vec<Fp> = Vec::with_capacity(window_size * width);
    for k in 0..window_size {
        let zk = z * g.pow(k as u64);
        for coeffs_c in &trace_coeffs {
            ood_frame.push(eval(coeffs_c, zk));
        }
    }
    for value in &ood_frame {
        transcript.absorb_fp(*value);
    }

    // The composition at z follows from the frame; a dishonest frame makes the
    // DEEP quotient below fail the low-degree test.
    let periodic_z: Vec<Fp> = {
        let h_pts: Vec<Fp> = {
            let mut v = Vec::with_capacity(t);
            let mut p = Fp::ONE;
            for _ in 0..t {
                v.push(p);
                p = p * g;
            }
            v
        };
        periodic_cols.iter().map(|col| super::super::poly::eval_lagrange(&h_pts, col, z)).collect()
    };
    let comp_z = compose(air, g, z, &ood_frame, &periodic_z, &coeffs);

    // DEEP coefficients: one per (column, window row) quotient and one for the
    // composition quotient.
    let deep_coeffs: Vec<Fp> =
        (0..width * window_size + 1).map(|_| transcript.challenge_fp()).collect();

    // The DEEP polynomial over the coset: every quotient is a polynomial exactly
    // when the committed values match the out-of-domain claims.
    let mut deep_d: Vec<Fp> = Vec::with_capacity(n);
    let mut x = shift;
    for j in 0..n {
        let mut acc = Fp::ZERO;
        for k in 0..window_size {
            let zk = z * g.pow(k as u64);
            let inv_x_zk = (x - zk).inv();
            for (c, column) in trace_d.iter().enumerate() {
                let claimed = ood_frame[k * width + c];
                acc = acc + deep_coeffs[k * width + c] * ((column[j] - claimed) * inv_x_zk);
            }
        }
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((comp_d[j] - comp_z) * (x - z).inv());
        deep_d.push(acc);
        x = x * omega;
    }

    // FRI proves the DEEP polynomial is low degree; its first root commits it.
    let fri = fri_prove(&deep_d, shift, fri_log_blowup, n_queries);
    let deep_tree = MerkleTree::commit(&deep_d);
    transcript.absorb_digest(&fri.roots[0]);

    // Consistency queries: open the DEEP value, each trace column, and the
    // composition at each sampled position.
    let mut queries: Vec<StarkQuery> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(n);
        let trace_vals: Vec<Fp> = trace_d.iter().map(|col| col[p]).collect();
        let trace_paths: Vec<Vec<[u8; 32]>> = trace_trees.iter().map(|tree| tree.open(p)).collect();
        queries.push(StarkQuery {
            deep: deep_d[p],
            deep_path: deep_tree.open(p),
            trace: trace_vals,
            trace_paths,
            comp: comp_d[p],
            comp_path: comp_tree.open(p),
        });
    }

    StarkProof { trace_roots, comp_root: comp_tree.root(), ood_frame, fri, queries }
}
