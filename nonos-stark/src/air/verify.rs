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

//! The DEEP STARK verifier, generic over any AIR. It recomputes the transcript,
//! evaluates the constraints once at the out-of-domain point using the claimed
//! trace frame, checks with FRI that the DEEP quotient polynomial is low degree,
//! and at each sampled position recomputes that quotient from the committed
//! trace and composition openings. A false trace either breaks a quotient (FRI
//! rejects) or disagrees with an opening (the consistency check rejects). It
//! only reads the proof and never panics.

use super::super::field::Fp;
use super::super::fri::{fri_verify, root_of_unity};
use super::super::merkle::verify_path;
use super::super::poly::eval_lagrange;
use super::super::transcript::Transcript;
use super::composition::{compose, domain_params, num_coeffs};
use super::prove::draw_ood_point;
use super::spec::Air;
use super::types::StarkProof;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Verify `proof` against `air`. The evaluation domain and low-degree bound are
/// derived from the AIR, matching the prover.
pub fn stark_verify<A: Air>(air: &A, proof: &StarkProof, n_queries: usize) -> bool {
    stark_verify_bound(air, proof, n_queries, &[])
}

/// Verify a proof bound to `context`. The context is absorbed into the transcript
/// before anything else, so a proof drawn under one context never verifies under
/// another. An empty context reduces to `stark_verify`. This is the primitive a
/// capsule attestation gates on, binding the membership proof to the capsule.
pub fn stark_verify_bound<A: Air>(
    air: &A,
    proof: &StarkProof,
    n_queries: usize,
    context: &[u8],
) -> bool {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params(air);
    let n = 1usize << log_n;
    let window_size = air.window_size();

    if proof.trace_roots.len() != width
        || proof.ood_frame.len() != window_size * width
        || proof.queries.len() != n_queries
    {
        return false;
    }

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    // Rebuild the transcript exactly as the prover did, binding the context first.
    let mut transcript = Transcript::new(b"NONOS-STARK");
    if !context.is_empty() {
        transcript.absorb_digest(&crate::hash::keccak256(context));
    }
    for root in &proof.trace_roots {
        transcript.absorb_digest(root);
    }
    let coeffs: Vec<Fp> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp()).collect();
    transcript.absorb_digest(&proof.comp_root);
    let z = draw_ood_point(&mut transcript, shift, n, t);
    for value in &proof.ood_frame {
        transcript.absorb_fp(*value);
    }
    let deep_coeffs: Vec<Fp> =
        (0..width * window_size + 1).map(|_| transcript.challenge_fp()).collect();

    // The constraints, checked once at the out-of-domain point: the composition
    // there is derived from the claimed frame with this verifier's own boundary
    // and transition algebra.
    let h_pts: Vec<Fp> = {
        let mut v = Vec::with_capacity(t);
        let mut p = Fp::ONE;
        for _ in 0..t {
            v.push(p);
            p = p * g;
        }
        v
    };
    let periodic_z: Vec<Fp> =
        air.periodic_columns().iter().map(|col| eval_lagrange(&h_pts, col, z)).collect();
    let comp_z = compose(air, g, z, &proof.ood_frame, &periodic_z, &coeffs);

    // The DEEP quotient polynomial must be low degree.
    if !fri_verify(&proof.fri, shift, log_n, fri_log_blowup, n_queries) {
        return false;
    }
    let deep_root = proof.fri.roots[0];
    transcript.absorb_digest(&deep_root);

    // Every sampled position: bind the openings to their commitments and
    // recompute the DEEP value from them.
    for qd in &proof.queries {
        let p = transcript.challenge_index(n);
        if qd.trace.len() != width || qd.trace_paths.len() != width {
            return false;
        }
        if !verify_path(&deep_root, p, qd.deep, &qd.deep_path)
            || !verify_path(&proof.comp_root, p, qd.comp, &qd.comp_path)
        {
            return false;
        }
        for c in 0..width {
            if !verify_path(&proof.trace_roots[c], p, qd.trace[c], &qd.trace_paths[c]) {
                return false;
            }
        }

        let x = shift * omega.pow(p as u64);
        let mut acc = Fp::ZERO;
        for k in 0..window_size {
            let zk = z * g.pow(k as u64);
            let inv_x_zk = (x - zk).inv();
            for c in 0..width {
                let claimed = proof.ood_frame[k * width + c];
                acc = acc + deep_coeffs[k * width + c] * ((qd.trace[c] - claimed) * inv_x_zk);
            }
        }
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((qd.comp - comp_z) * (x - z).inv());
        if acc != qd.deep {
            return false;
        }
    }

    true
}
