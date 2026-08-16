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

//! The money-grade DEEP STARK verifier. Same checks as the base verifier, done in
//! the extension: it evaluates the constraints once at the out-of-domain point
//! `z in Fp2`, verifies the DEEP quotient is low degree with the money-grade FRI,
//! and at each sampled position recomputes that quotient from the committed trace
//! (base leaf) and composition (extension leaf) openings. It only reads the proof
//! and never panics.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_ext::fri_verify_ext;
use super::super::merkle::{verify_path_ext, verify_path_wide};
use super::super::poly::eval_subgroup_or_lagrange_ext;
use super::super::transcript::Transcript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::prove_ext::draw_ood_point_ext;
use super::spec::AirExt;
use super::types_ext::StarkProofExt;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Verify a money-grade `proof` against `air`. Domain sizing matches the prover.
pub fn stark_verify_ext<A: AirExt>(
    air: &A,
    proof: &StarkProofExt,
    n_queries: usize,
    grind_bits: u32,
) -> bool {
    stark_verify_ext_blown(air, proof, n_queries, grind_bits, 0)
}

/// The same verifier, with `extra_blowup_bits` matching the prover's. A deployment
/// verifier pins this to the value the vector was proven at.
pub fn stark_verify_ext_blown<A: AirExt>(
    air: &A,
    proof: &StarkProofExt,
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> bool {
    stark_verify_ext_blown_bound(air, proof, n_queries, grind_bits, extra_blowup_bits, &[])
}

/// The same verifier bound to `context`, absorbed into the transcript first, so a
/// proof drawn under one context never verifies under another. The money-grade
/// attestation verifier.
pub fn stark_verify_ext_blown_bound<A: AirExt>(
    air: &A,
    proof: &StarkProofExt,
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    context: &[u8],
) -> bool {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let window_size = air.window_size();

    if proof.ood_frame.len() != window_size * width || proof.queries.len() != n_queries {
        return false;
    }

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    if !context.is_empty() {
        transcript.absorb_digest(&crate::hash::keccak256(context));
    }
    transcript.absorb_digest(&proof.trace_root);
    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp2()).collect();
    transcript.absorb_digest(&proof.comp_root);
    let z = draw_ood_point_ext(&mut transcript, shift, n, t);
    for value in &proof.ood_frame {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let deep_coeffs: Vec<Fp2> =
        (0..width * window_size + 1).map(|_| transcript.challenge_fp2()).collect();

    // The constraints checked once at the out-of-domain point from the claimed
    // frame, with this verifier's own composition algebra.
    let h_pts: Vec<Fp> = {
        let mut v = Vec::with_capacity(t);
        let mut p = Fp::ONE;
        for _ in 0..t {
            v.push(p);
            p = p * g;
        }
        v
    };
    let periodic_z: Vec<Fp2> = air
        .periodic_columns()
        .iter()
        .map(|col| eval_subgroup_or_lagrange_ext(g, &h_pts, col, z))
        .collect();
    let comp_z = compose_ext(air, g, z, &proof.ood_frame, &periodic_z, &coeffs);

    // The DEEP quotient polynomial must be low degree.
    if !fri_verify_ext(&proof.fri, shift, log_n, fri_log_blowup, n_queries, grind_bits) {
        return false;
    }
    let deep_root = proof.fri.roots[0];
    transcript.absorb_digest(&deep_root);

    for qd in &proof.queries {
        let p = transcript.challenge_index(n);
        if qd.trace.len() != width {
            return false;
        }
        if !verify_path_ext(&deep_root, p, qd.deep, &qd.deep_path)
            || !verify_path_ext(&proof.comp_root, p, qd.comp, &qd.comp_path)
            || !verify_path_wide(&proof.trace_root, p, &qd.trace, &qd.trace_path)
        {
            return false;
        }

        let x = shift * omega.pow(p as u64);
        let mut acc = Fp2::ZERO;
        for k in 0..window_size {
            let zk = z * Fp2::from_base(g.pow(k as u64));
            let inv_x_zk = (Fp2::from_base(x) - zk).inv();
            for c in 0..width {
                let claimed = proof.ood_frame[k * width + c];
                acc = acc
                    + deep_coeffs[k * width + c]
                        * ((Fp2::from_base(qd.trace[c]) - claimed) * inv_x_zk);
            }
        }
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((qd.comp - comp_z) * (Fp2::from_base(x) - z).inv());
        if acc != qd.deep {
            return false;
        }
    }

    true
}
