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

//! The deployment prover with the preprocessed-periodic sidecar. Identical to
//! the money-grade prover through the out-of-domain frame; then the periodic
//! evaluations at z are absorbed as claims, the DEEP coefficient draw is
//! extended by one per periodic column, and the DEEP polynomial gains a
//! quotient term per periodic column against its claim. The periodic columns'
//! coset extension is committed row-wise under the periodic wide-leaf domain
//! (the root a verifier bakes as a constant), and every consistency query
//! opens its whole periodic row with one path. A verifier then never
//! reconstructs the structural columns: it authenticates the openings and the
//! DEEP low-degree argument binds the claims.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_ext::fri_prove_ext;
use super::super::merkle::MerkleTree;
use super::super::poly::{eval_ext, eval_subgroup_or_lagrange_ext, intt, lde};
use super::super::transcript::Transcript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::prove_ext::draw_ood_point_ext;
use super::spec::AirExt;
use super::types_ext::{StarkProofExt, StarkQueryExt};
use super::types_ext_pre::{PeriodicOpeningExt, StarkProofExtPre};
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Prove `trace` against `air` with the periodic sidecar, at the given FRI
/// rate. The verifier must hold the matching baked periodic root.
pub fn stark_prove_ext_preprocessed<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> StarkProofExtPre {
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

    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    // Each column's interpolation and coset extension are independent of the
    // others, so the trace LDE parallelizes over columns; the indexed map keeps
    // column order, so the committed tree is identical to the serial one.
    let cols: Vec<(Vec<Fp>, Vec<Fp>)> = crate::par::map_index(width, |c| {
        let column: Vec<Fp> = (0..t).map(|i| trace[i * width + c]).collect();
        (intt(&column, g), lde(&column, g, shift, omega, n))
    });
    let (trace_coeffs, trace_d): (Vec<Vec<Fp>>, Vec<Vec<Fp>>) = cols.into_iter().unzip();
    let trace_tree = MerkleTree::commit_wide(&trace_d);
    let trace_root = trace_tree.root();
    transcript.absorb_digest(&trace_root);

    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp2()).collect();

    let periodic_cols = air.periodic_columns();
    let n_periodic = periodic_cols.len();
    // The periodic coset extension and its wide-periodic tree come from the shared
    // helper, the same one the verifier-key root helper uses, so a registered root
    // and the root committed here are the same object by construction.
    let (periodic_d, periodic_tree) =
        super::periodic_root::periodic_lde_tree(air, extra_blowup_bits);

    // The composition value at each coset point is independent: the evaluation
    // point is x_j = shift * omega^j, a pure function of j, so the whole
    // codeword parallelizes over the domain.
    let comp_d: Vec<Fp2> = crate::par::map_index(n, |j| {
        let mut window: Vec<Fp2> = Vec::with_capacity(window_size * width);
        for k in 0..window_size {
            let idx = (j + k * blowup) % n;
            for column in &trace_d {
                window.push(Fp2::from_base(column[idx]));
            }
        }
        let periodic: Vec<Fp2> = periodic_d.iter().map(|pd| Fp2::from_base(pd[j])).collect();
        let x = shift * omega.pow(j as u64);
        compose_ext(air, g, Fp2::from_base(x), &window, &periodic, &coeffs)
    });
    let comp_tree = MerkleTree::commit_ext(&comp_d);
    transcript.absorb_digest(&comp_tree.root());

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

    // The periodic evaluations at z: claims the sidecar carries, absorbed
    // before the DEEP coefficients so the low-degree argument binds them.
    let h_pts: Vec<Fp> = {
        let mut v = Vec::with_capacity(t);
        let mut p = Fp::ONE;
        for _ in 0..t {
            v.push(p);
            p = p * g;
        }
        v
    };
    let periodic_z: Vec<Fp2> =
        periodic_cols.iter().map(|col| eval_subgroup_or_lagrange_ext(g, &h_pts, col, z)).collect();
    for value in &periodic_z {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let comp_z = compose_ext(air, g, z, &ood_frame, &periodic_z, &coeffs);

    // DEEP coefficients: the trace terms, the composition term, then one per
    // periodic column.
    let deep_coeffs: Vec<Fp2> =
        (0..width * window_size + 1 + n_periodic).map(|_| transcript.challenge_fp2()).collect();

    // The DEEP quotient at each coset point is likewise independent of the
    // others (same x_j = shift * omega^j), so it parallelizes over the domain.
    let deep_d: Vec<Fp2> = crate::par::map_index(n, |j| {
        let x = shift * omega.pow(j as u64);
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
        let inv_x_z = (Fp2::from_base(x) - z).inv();
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((comp_d[j] - comp_z) * inv_x_z);
        for (pi, pd) in periodic_d.iter().enumerate() {
            let pc = deep_coeffs[width * window_size + 1 + pi];
            acc = acc + pc * ((Fp2::from_base(pd[j]) - periodic_z[pi]) * inv_x_z);
        }
        acc
    });

    let fri = fri_prove_ext(&deep_d, shift, fri_log_blowup, n_queries, grind_bits);
    let deep_tree = MerkleTree::commit_ext(&deep_d);
    transcript.absorb_digest(&fri.roots[0]);

    let mut queries: Vec<StarkQueryExt> = Vec::with_capacity(n_queries);
    let mut openings: Vec<PeriodicOpeningExt> = Vec::with_capacity(n_queries);
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
        openings.push(PeriodicOpeningExt {
            row: periodic_d.iter().map(|pd| pd[p]).collect(),
            path: periodic_tree.open(p),
        });
    }

    StarkProofExtPre {
        proof: StarkProofExt { trace_root, comp_root: comp_tree.root(), ood_frame, fri, queries },
        periodic_z,
        openings,
    }
}
