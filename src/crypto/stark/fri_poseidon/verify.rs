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

//! The Poseidon-committed FRI verifier. Same checks as the BLAKE3 verifier, over
//! Poseidon Merkle openings and a Poseidon transcript, so this exact procedure
//! can later be arithmetized as an AIR.

use super::super::air::Poseidon;
use super::super::field::Fp;
use super::super::fri::root_of_unity;
use super::super::poseidon_merkle::verify_path;
use super::super::poseidon_transcript::PoseidonTranscript;
use super::prove::leaf;
use super::types::FriProof;
use alloc::vec::Vec;

pub fn fri_verify(
    proof: &FriProof,
    shift: Fp,
    log_n: u32,
    log_blowup: u32,
    n_queries: usize,
    hasher: &Poseidon,
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

    let mut transcript = PoseidonTranscript::new(hasher.clone());
    let mut betas: Vec<Fp> = Vec::with_capacity(n_folds);
    for root in &proof.roots {
        transcript.absorb_digest(root);
        betas.push(transcript.challenge());
    }

    let final_value = proof.final_layer[0];
    for value in &proof.final_layer {
        if *value != final_value {
            return false;
        }
        transcript.absorb(*value);
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

            if !verify_path(hasher, &proof.roots[m], i, leaf(op.a), &op.a_path)
                || !verify_path(hasher, &proof.roots[m], i + half, leaf(op.b), &op.b_path)
            {
                return false;
            }

            let x = (shift * base_omega.pow(i as u64)).pow(1u64 << m);
            let even = (op.a + op.b) * inv2;
            let odd = (op.a - op.b) * inv2 * x.inv();
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
