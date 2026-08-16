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

//! Extracting the out-of-domain composition inputs of a Poseidon-committed proof:
//! the out-of-domain frame is the proof's, and the batching coefficients, the
//! out-of-domain point, and the periodic columns at that point are recovered by
//! replaying the transcript exactly as the verifier does. These are the inputs a
//! recursive verifier feeds to an in-circuit `compose_ext`, so it can prove the
//! composition value the DEEP check consumes was honestly formed.

use super::super::air::Poseidon;
use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::poly::eval_subgroup_or_lagrange_ext;
use super::super::poseidon_transcript::PoseidonTranscript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::spec::AirExt;
use super::types_poseidon_ext::StarkProofExtP;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// The composition inputs at the out-of-domain point: the batching coefficients,
/// the periodic columns evaluated at `z`, the point `z`, and the resulting
/// composition value. The out-of-domain frame is `proof.ood_frame` directly.
pub struct ComposeInputs {
    pub coeffs: Vec<Fp2>,
    pub periodic_z: Vec<Fp2>,
    pub z: Fp2,
    pub comp_z: Fp2,
}

/// Replay the transcript up to the out-of-domain challenges and assemble the
/// composition inputs of `proof` under `air`.
pub fn compose_inputs<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
) -> ComposeInputs {
    compose_inputs_pub(air, proof, extra_blowup_bits, hasher, &[])
}

/// The same extraction, replaying `publics` into the transcript first, matching a
/// proof produced with `stark_prove_poseidon_ext_pub`.
pub fn compose_inputs_pub<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
) -> ComposeInputs {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let (log_n, _) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;

    let g = root_of_unity(log_t);
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

    ComposeInputs { coeffs, periodic_z, z, comp_z }
}
