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

//! The Poseidon-committed money-grade STARK verifier: the exact algorithm of
//! `verify_ext`, but replaying the Poseidon transcript and checking Poseidon Merkle
//! openings. It is the algorithm a recursive verifier arithmetizes, so a proof that
//! verifies here is the proof a recursion attests to.

use super::super::air::Poseidon;
use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_poseidon_ext::fri_verify_poseidon_ext;
use super::super::poly::eval_subgroup_or_lagrange_ext;
use super::super::poseidon_merkle::{pack_base, pack_ext, verify_path};
use super::super::poseidon_transcript::PoseidonTranscript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::spec::AirExt;
use super::types_poseidon_ext::StarkProofExtP;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

pub fn stark_verify_poseidon_ext<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
) -> bool {
    stark_verify_poseidon_ext_pub(air, proof, n_queries, grind_bits, extra_blowup_bits, hasher, &[])
}

/// The same verifier, seeding the transcript with `publics` before the trace roots
/// so acceptance is bound to those public inputs by Fiat-Shamir. It is the exact
/// counterpart of `stark_prove_poseidon_ext_pub`: replaying the same seed in the
/// same order. A recursive verifier exposes `publics` in its transcript column, so
/// this is the algorithm a recursion over a public-bound proof arithmetizes.
#[allow(clippy::too_many_arguments)]
pub fn stark_verify_poseidon_ext_pub<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
) -> bool {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params_blown(air, extra_blowup_bits);
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

    let mut transcript = PoseidonTranscript::new(hasher.clone());
    for &p in publics {
        transcript.absorb(p);
    }
    for root in &proof.trace_roots {
        transcript.absorb_digest(root);
    }
    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp2()).collect();
    transcript.absorb_digest(&proof.comp_root);
    let z = draw_ood_point_poseidon(&mut transcript, shift, n, t);
    for value in &proof.ood_frame {
        transcript.absorb(value.c0);
        transcript.absorb(value.c1);
    }
    let deep_coeffs: Vec<Fp2> =
        (0..width * window_size + 1).map(|_| transcript.challenge_fp2()).collect();

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

    if !fri_verify_poseidon_ext(
        &proof.fri,
        shift,
        log_n,
        fri_log_blowup,
        n_queries,
        grind_bits,
        hasher,
    ) {
        return false;
    }
    let deep_root = proof.fri.roots[0];
    transcript.absorb_digest(&deep_root);

    for qd in &proof.queries {
        let p = transcript.challenge_index(n);
        if qd.trace.len() != width || qd.trace_paths.len() != width {
            return false;
        }
        if !verify_path(hasher, &deep_root, p, pack_ext(qd.deep), &qd.deep_path)
            || !verify_path(hasher, &proof.comp_root, p, pack_ext(qd.comp), &qd.comp_path)
        {
            return false;
        }
        for c in 0..width {
            if !verify_path(
                hasher,
                &proof.trace_roots[c],
                p,
                pack_base(qd.trace[c]),
                &qd.trace_paths[c],
            ) {
                return false;
            }
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
