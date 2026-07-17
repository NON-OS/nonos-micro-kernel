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

//! Extracting a Poseidon-committed proof's query-zero authentication openings: the
//! DEEP value against the FRI root, the composition against the composition root,
//! and every trace column against its trace root, all at the consistency query
//! position. These are exactly the openings the inner verifier authenticates before
//! it trusts the DEEP algebra (`verify_poseidon_ext` runs the same three checks). A
//! recursive verifier authenticates them with one batched membership region, closing
//! the gap where the opened values feeding the DEEP check were trusted rather than
//! proven to be the committed ones. The batch is equal depth because the trace, the
//! composition, and the DEEP polynomial are all committed over the same domain.

use super::super::field::{Fp, Fp2};
use super::super::poseidon_merkle::{pack_base, pack_ext};
use super::super::poseidon_transcript::PoseidonTranscript;
use super::composition::{domain_params_blown, num_coeffs};
use super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::multi_membership::Opening;
use super::spec::AirExt;
use super::types_poseidon_ext::StarkProofExtP;
use super::Poseidon;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// The query-zero openings, replaying the transcript to the consistency query index
/// exactly as `stark_verify_poseidon_ext` does, then packaging each authenticated
/// opening (leaf, root, path, directions) the same batched membership consumes.
pub fn query_openings_query0<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
) -> Vec<Opening> {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let window_size = air.window_size();
    let (log_n, _) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let shift = Fp::from_u64(SHIFT);

    let mut ts = PoseidonTranscript::new(hasher.clone());
    for &p in publics {
        ts.absorb(p);
    }
    for root in &proof.trace_roots {
        ts.absorb_digest(root);
    }
    let _coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| ts.challenge_fp2()).collect();
    ts.absorb_digest(&proof.comp_root);
    let _z = draw_ood_point_poseidon(&mut ts, shift, n, t);
    for value in &proof.ood_frame {
        ts.absorb(value.c0);
        ts.absorb(value.c1);
    }
    let _deep_coeffs: Vec<Fp2> = (0..width * window_size + 1).map(|_| ts.challenge_fp2()).collect();
    ts.absorb_digest(&proof.fri.roots[0]);
    let p = ts.challenge_index(n);

    let qd = &proof.queries[0];
    let depth = qd.deep_path.len();
    let directions: Vec<bool> = (0..depth).map(|lv| (p >> lv) & 1 == 1).collect();

    let mut openings = Vec::with_capacity(width + 2);
    // The DEEP value against the FRI root (the same authentication the fold's opened
    // value already relies on), the composition against the composition root, then
    // every trace column against its own trace root.
    openings.push(Opening {
        leaf: pack_ext(qd.deep),
        root: proof.fri.roots[0],
        siblings: qd.deep_path.clone(),
        directions: directions.clone(),
    });
    openings.push(Opening {
        leaf: pack_ext(qd.comp),
        root: proof.comp_root,
        siblings: qd.comp_path.clone(),
        directions: directions.clone(),
    });
    for c in 0..width {
        openings.push(Opening {
            leaf: pack_base(qd.trace[c]),
            root: proof.trace_roots[c],
            siblings: qd.trace_paths[c].clone(),
            directions: directions.clone(),
        });
    }
    openings
}
