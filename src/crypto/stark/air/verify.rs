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

//! The STARK verifier. It checks the composition is low degree through FRI, then
//! at each sampled position checks the committed composition equals the
//! constraint composition recomputed from the committed trace. A false trace
//! yields either a high-degree composition (FRI rejects) or a composition that
//! disagrees with the trace (consistency rejects). It only reads the proof.

use super::super::field::Fp;
use super::super::fri::{fri_verify, root_of_unity};
use super::super::merkle::verify_path;
use super::super::transcript::Transcript;
use super::constraints::{quotients, AirParams};
use super::types::StarkProof;

const SHIFT: u64 = 7;

/// Verify a squaring-AIR STARK proof for a trace of length `2^log_t` on an
/// evaluation domain `2^log_blowup` times larger, with public boundary `seed`.
pub fn stark_verify(
    proof: &StarkProof,
    seed: Fp,
    log_t: u32,
    log_blowup: u32,
    n_queries: usize,
) -> bool {
    let t = 1usize << log_t;
    let log_n = log_t + log_blowup;
    let n = 1usize << log_n;
    let blowup = 1usize << log_blowup;

    if proof.queries.len() != n_queries {
        return false;
    }

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);
    let params = AirParams { log_t, seed, g_last: g.pow((t - 1) as u64) };

    // 1. The composition must be low degree (below the trace length).
    if !fri_verify(&proof.fri, shift, log_n, log_blowup, n_queries) {
        return false;
    }
    let comp_root = proof.fri.roots[0];

    // 2. Recover the composition coefficients and the query positions.
    let mut transcript = Transcript::new(b"NONOS-STARK");
    transcript.absorb_digest(&proof.trace_root);
    let alpha = transcript.challenge_fp();
    let beta = transcript.challenge_fp();
    transcript.absorb_digest(&comp_root);

    // 3. The committed composition must equal the constraint composition of the
    // committed trace at every sampled position.
    for qd in &proof.queries {
        let p = transcript.challenge_index(n);
        let gx = (p + blowup) % n;
        if !verify_path(&comp_root, p, qd.comp, &qd.comp_path)
            || !verify_path(&proof.trace_root, p, qd.t_x, &qd.t_x_path)
            || !verify_path(&proof.trace_root, gx, qd.t_gx, &qd.t_gx_path)
        {
            return false;
        }
        let x = shift * omega.pow(p as u64);
        let (q_transition, q_boundary) = quotients(&params, x, qd.t_x, qd.t_gx);
        if qd.comp != alpha * q_transition + beta * q_boundary {
            return false;
        }
    }

    true
}
