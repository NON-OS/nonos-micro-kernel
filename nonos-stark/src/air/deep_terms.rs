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

//! Extracting the DEEP-consistency witness of a Poseidon-committed proof's first
//! query: replay the transcript exactly as the verifier does, recover the
//! out-of-domain point, the batching coefficients, and the query position, then
//! assemble the per-term data (opened value, out-of-domain claim, point,
//! coefficient) a recursive verifier feeds to `DeepCheckExt`. This is the bridge
//! that turns a real proof into the terms an in-circuit DEEP check consumes.

use super::super::air::Poseidon;
use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::poly::eval_subgroup_or_lagrange_ext;
use super::super::poseidon_transcript::PoseidonTranscript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::deep_check_ext::DeepTerm;
use super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::spec::AirExt;
use super::types_poseidon_ext::StarkProofExtP;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// The DEEP terms of `proof`'s first query, its evaluation point, and the query's
/// DEEP value. The transcript replay mirrors `stark_verify_poseidon_ext` up to the
/// first query index.
pub fn deep_terms_query0<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
) -> (Vec<DeepTerm>, Fp2, Fp2) {
    deep_terms_query0_pub(air, proof, extra_blowup_bits, hasher, &[])
}

/// The same extraction, replaying `publics` into the transcript first.
pub fn deep_terms_query0_pub<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
) -> (Vec<DeepTerm>, Fp2, Fp2) {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, _) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let window_size = air.window_size();

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    let mut ts = PoseidonTranscript::new(hasher.clone());
    for &p in publics {
        ts.absorb(p);
    }
    for root in &proof.trace_roots {
        ts.absorb_digest(root);
    }
    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| ts.challenge_fp2()).collect();
    ts.absorb_digest(&proof.comp_root);
    let z = draw_ood_point_poseidon(&mut ts, shift, n, t);
    for value in &proof.ood_frame {
        ts.absorb(value.c0);
        ts.absorb(value.c1);
    }
    let deep_coeffs: Vec<Fp2> = (0..width * window_size + 1).map(|_| ts.challenge_fp2()).collect();

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

    ts.absorb_digest(&proof.fri.roots[0]);
    let p = ts.challenge_index(n);

    let qd = &proof.queries[0];
    let x = Fp2::from_base(shift * omega.pow(p as u64));

    let mut terms: Vec<DeepTerm> = Vec::with_capacity(width * window_size + 1);
    for k in 0..window_size {
        let zk = z * Fp2::from_base(g.pow(k as u64));
        for c in 0..width {
            terms.push(DeepTerm {
                val: Fp2::from_base(qd.trace[c]),
                claim: proof.ood_frame[k * width + c],
                point: zk,
                coeff: deep_coeffs[k * width + c],
            });
        }
    }
    terms.push(DeepTerm {
        val: qd.comp,
        claim: comp_z,
        point: z,
        coeff: deep_coeffs[width * window_size],
    });

    (terms, x, qd.deep)
}
