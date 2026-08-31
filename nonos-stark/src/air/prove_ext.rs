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

//! The money-grade DEEP STARK prover. Same construction as the base prover, but
//! the composition batching, the out-of-domain point, and every downstream
//! quotient live in the extension `Fp2`, and the low-degree test is the money-grade
//! FRI (`fri_ext`) with grinding. The trace is committed in the base field; the
//! composition and the DEEP polynomial are committed over the extension.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_ext::fri_prove_ext;
use super::super::merkle::MerkleTree;
use super::super::poly::{eval_ext, eval_subgroup_or_lagrange_ext, intt, lde};
use super::super::transcript::Transcript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::spec::AirExt;
use super::types_ext::{StarkProofExt, StarkQueryExt};
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Draw the out-of-domain point from the extension: off the evaluation coset and
/// off the trace domain, so every DEEP and periodic denominator is invertible in
/// `Fp2`. Both sides run this identically, so they agree on the point.
pub(super) fn draw_ood_point_ext(
    transcript: &mut Transcript,
    shift: Fp,
    n: usize,
    t: usize,
) -> Fp2 {
    let shift_n = Fp2::from_base(shift.pow(n as u64));
    let mut z = transcript.challenge_fp2();
    while z.pow(n as u64) == shift_n || z.pow(t as u64) == Fp2::ONE {
        z = transcript.challenge_fp2();
    }
    z
}

/// Prove that `trace` satisfies `air` at money-grade soundness. Layout and domain
/// sizing match the base prover; `grind_bits` is the FRI proof-of-work.
pub fn stark_prove_ext<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
) -> StarkProofExt {
    stark_prove_ext_blown(air, trace, n_queries, grind_bits, 0)
}

/// The same prover, with `extra_blowup_bits` of FRI low-degree headroom. Zero is
/// the minimal rate-one-half domain used everywhere in tests; a deployment vector
/// raises it so a fixed query count reaches 128-bit soundness. The verifier must be
/// given the same value, since it recomputes the domain from the AIR.
pub fn stark_prove_ext_blown<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> StarkProofExt {
    stark_prove_ext_blown_bound(air, trace, n_queries, grind_bits, extra_blowup_bits, &[])
}

/// The same prover bound to `context`, which is absorbed into the transcript before
/// anything else, so the proof only verifies under the same context. This is the
/// money-grade attestation prover: the extension challenges and the raised FRI rate
/// give 128-bit soundness with the proof pinned to the capsule identity.
pub fn stark_prove_ext_blown_bound<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    context: &[u8],
) -> StarkProofExt {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let blowup = 1usize << (log_n - log_t);
    let window_size = air.window_size();

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    // Trace columns are extended onto the coset, then committed row-wise as one
    // wide-leaf tree: leaf i hashes every column at row i, so a query authenticates
    // the whole row with a single path and the transcript absorbs one root.
    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    if !context.is_empty() {
        transcript.absorb_digest(&crate::hash::keccak256(context));
    }
    let mut trace_d: Vec<Vec<Fp>> = Vec::with_capacity(width);
    let mut trace_coeffs: Vec<Vec<Fp>> = Vec::with_capacity(width);
    for c in 0..width {
        let column: Vec<Fp> = (0..t).map(|i| trace[i * width + c]).collect();
        trace_coeffs.push(intt(&column, g));
        trace_d.push(lde(&column, g, shift, omega, n));
    }
    let trace_tree = MerkleTree::commit_wide(&trace_d);
    let trace_root = trace_tree.root();
    transcript.absorb_digest(&trace_root);

    // Composition batching coefficients, drawn from the extension for soundness.
    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp2()).collect();

    let periodic_cols = air.periodic_columns();
    let periodic_d: Vec<Vec<Fp>> =
        periodic_cols.iter().map(|col| lde(col, g, shift, omega, n)).collect();

    // The composition over the coset, in the extension: base window lifted, batched
    // under the extension coefficients.
    let mut comp_d: Vec<Fp2> = Vec::with_capacity(n);
    let mut x = shift;
    for j in 0..n {
        let mut window: Vec<Fp2> = Vec::with_capacity(window_size * width);
        for k in 0..window_size {
            let idx = (j + k * blowup) % n;
            for column in &trace_d {
                window.push(Fp2::from_base(column[idx]));
            }
        }
        let periodic: Vec<Fp2> = periodic_d.iter().map(|pd| Fp2::from_base(pd[j])).collect();
        comp_d.push(compose_ext(air, g, Fp2::from_base(x), &window, &periodic, &coeffs));
        x = x * omega;
    }
    let comp_tree = MerkleTree::commit_ext(&comp_d);
    transcript.absorb_digest(&comp_tree.root());

    // The out-of-domain point in the extension, and the trace frame there.
    let z = draw_ood_point_ext(&mut transcript, shift, n, t);
    let mut ood_frame: Vec<Fp2> = Vec::with_capacity(window_size * width);
    for k in 0..window_size {
        let zk = z * Fp2::from_base(g.pow(k as u64));
        for coeffs_c in &trace_coeffs {
            ood_frame.push(eval_ext(coeffs_c, zk));
        }
    }
    for value in &ood_frame {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }

    // The composition at z, from the claimed frame.
    let periodic_z: Vec<Fp2> = {
        let h_pts: Vec<Fp> = {
            let mut v = Vec::with_capacity(t);
            let mut p = Fp::ONE;
            for _ in 0..t {
                v.push(p);
                p = p * g;
            }
            v
        };
        periodic_cols.iter().map(|col| eval_subgroup_or_lagrange_ext(g, &h_pts, col, z)).collect()
    };
    let comp_z = compose_ext(air, g, z, &ood_frame, &periodic_z, &coeffs);

    // DEEP batching coefficients, also from the extension.
    let deep_coeffs: Vec<Fp2> =
        (0..width * window_size + 1).map(|_| transcript.challenge_fp2()).collect();

    // The DEEP polynomial over the coset, in the extension.
    let mut deep_d: Vec<Fp2> = Vec::with_capacity(n);
    let mut x = shift;
    for j in 0..n {
        let mut acc = Fp2::ZERO;
        for k in 0..window_size {
            let zk = z * Fp2::from_base(g.pow(k as u64));
            let inv_x_zk = (Fp2::from_base(x) - zk).inv();
            for (c, column) in trace_d.iter().enumerate() {
                let claimed = ood_frame[k * width + c];
                acc = acc
                    + deep_coeffs[k * width + c]
                        * ((Fp2::from_base(column[j]) - claimed) * inv_x_zk);
            }
        }
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((comp_d[j] - comp_z) * (Fp2::from_base(x) - z).inv());
        deep_d.push(acc);
        x = x * omega;
    }

    // The money-grade FRI proves the DEEP polynomial is low degree; its layer-zero
    // root also commits the DEEP values the consistency queries open.
    let fri = fri_prove_ext(&deep_d, shift, fri_log_blowup, n_queries, grind_bits);
    let deep_tree = MerkleTree::commit_ext(&deep_d);
    transcript.absorb_digest(&fri.roots[0]);

    let mut queries: Vec<StarkQueryExt> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(n);
        let trace_vals: Vec<Fp> = trace_d.iter().map(|col| col[p]).collect();
        queries.push(StarkQueryExt {
            deep: deep_d[p],
            deep_path: deep_tree.open(p),
            trace: trace_vals,
            trace_path: trace_tree.open(p),
            comp: comp_d[p],
            comp_path: comp_tree.open(p),
        });
    }

    StarkProofExt { trace_root, comp_root: comp_tree.root(), ood_frame, fri, queries }
}
