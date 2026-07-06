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

//! The STARK verifier, generic over any AIR. It checks the composition is low
//! degree through FRI, then at each sampled position checks the committed
//! composition equals the constraint composition recomputed from the committed
//! trace window. A false trace yields either a high-degree composition (FRI
//! rejects) or one that disagrees with the trace (consistency rejects). It only
//! reads the proof and never panics.

use super::super::field::Fp;
use super::super::fri::{fri_verify, root_of_unity};
use super::super::merkle::verify_path;
use super::super::transcript::Transcript;
use super::composition::{compose, num_coeffs};
use super::spec::Air;
use super::types::StarkProof;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Verify `proof` against `air` on an evaluation domain `2^log_blowup` times the
/// trace length.
pub fn stark_verify<A: Air>(
    air: &A,
    proof: &StarkProof,
    log_blowup: u32,
    n_queries: usize,
) -> bool {
    let log_t = air.log_trace_len();
    let log_n = log_t + log_blowup;
    let n = 1usize << log_n;
    let blowup = 1usize << log_blowup;
    let window_size = air.window_size();

    if proof.queries.len() != n_queries {
        return false;
    }

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    // 1. The composition must be low degree (below the trace length).
    if !fri_verify(&proof.fri, shift, log_n, log_blowup, n_queries) {
        return false;
    }
    let comp_root = proof.fri.roots[0];

    // 2. Recover the composition coefficients and query positions.
    let mut transcript = Transcript::new(b"NONOS-STARK");
    transcript.absorb_digest(&proof.trace_root);
    let coeffs: Vec<Fp> = (0..num_coeffs(air)).map(|_| transcript.challenge_fp()).collect();
    transcript.absorb_digest(&comp_root);

    // 3. The committed composition must equal the constraint composition of the
    // committed trace window at every sampled position.
    for qd in &proof.queries {
        let p = transcript.challenge_index(n);
        if qd.window.len() != window_size || qd.window_paths.len() != window_size {
            return false;
        }
        if !verify_path(&comp_root, p, qd.comp, &qd.comp_path) {
            return false;
        }
        for k in 0..window_size {
            let idx = (p + k * blowup) % n;
            if !verify_path(&proof.trace_root, idx, qd.window[k], &qd.window_paths[k]) {
                return false;
            }
        }
        let x = shift * omega.pow(p as u64);
        if qd.comp != compose(air, g, x, &qd.window, &coeffs) {
            return false;
        }
    }

    true
}
