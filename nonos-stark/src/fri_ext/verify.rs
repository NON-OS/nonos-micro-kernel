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

//! The money-grade FRI verifier. It rebuilds the transcript, draws each fold
//! challenge from the extension, checks the grinding proof-of-work, and for every
//! sampled query re-derives the extension fold and binds each opened value by a
//! Merkle path. It only reads the proof and never panics.

use super::super::field::Fp;
use super::super::field::Fp2;
use super::super::fri::root_of_unity;
use super::super::merkle::verify_path_ext;
use super::super::transcript::Transcript;
use super::types::FriProofExt;
use alloc::vec::Vec;

/// Verify a money-grade FRI proof. Returns `true` only if every structural,
/// grinding, Merkle, folding, and low-degree check passes for all queries.
pub fn fri_verify_ext(
    proof: &FriProofExt,
    shift: Fp,
    log_n: u32,
    log_blowup: u32,
    n_queries: usize,
    grind_bits: u32,
) -> bool {
    let n = 1usize << log_n;
    let blowup = 1usize << log_blowup;
    let n_folds = (log_n - log_blowup) as usize;

    if proof.roots.len() != n_folds
        || proof.final_layer.len() != blowup
        || proof.queries.len() != n_queries
    {
        return false;
    }

    let base_omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();

    let mut transcript = Transcript::new(b"NONOS-STARK-FRI-EXT");
    let mut betas: Vec<Fp2> = Vec::with_capacity(n_folds);
    for root in &proof.roots {
        transcript.absorb_digest(root);
        betas.push(transcript.challenge_fp2());
    }

    // The low-degree conclusion: the final layer must be a single constant.
    let final_value = proof.final_layer[0];
    for value in &proof.final_layer {
        if *value != final_value {
            return false;
        }
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }

    // The grinding nonce must meet the proof-of-work, bound at the same transcript
    // point the prover committed it, before any query position is drawn.
    if !transcript.verify_pow(proof.pow_nonce, grind_bits) {
        return false;
    }

    for qp in &proof.queries {
        if qp.layers.len() != n_folds {
            return false;
        }
        let q = transcript.challenge_index(n);
        for (m, beta) in betas.iter().enumerate() {
            let half = n >> (m + 1);
            let i = q % half;
            let op = &qp.layers[m];

            if !verify_path_ext(&proof.roots[m], i, op.a, &op.a_path)
                || !verify_path_ext(&proof.roots[m], i + half, op.b, &op.b_path)
            {
                return false;
            }

            // The queried evaluation point on the squared coset, in the base field.
            let x = (shift * base_omega.pow(i as u64)).pow(1u64 << m);
            let even = (op.a + op.b).mul_base(inv2);
            let odd = (op.a - op.b).mul_base(inv2).mul_base(x.inv());
            let folded = even + *beta * odd;

            if m + 1 < n_folds {
                let half_next = n >> (m + 2);
                let next = &qp.layers[m + 1];
                let expected = if i < half_next { next.a } else { next.b };
                if folded != expected {
                    return false;
                }
            } else if folded != final_value {
                return false;
            }
        }
    }

    true
}
