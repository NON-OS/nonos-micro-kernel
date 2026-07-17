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

//! The Poseidon-committed money-grade FRI prover: the same extension-field fold as
//! `fri_ext`, but every layer is committed with a Poseidon Merkle tree and the
//! transcript is the Poseidon sponge, so a proof made here can have its Merkle
//! openings, folds, and challenges re-derived inside a STARK. That is what makes it
//! the inner form for recursion.

use super::super::air::{Poseidon, RATE};
use super::super::field::{Fp, Fp2};
use super::super::fri::{fold_ext, root_of_unity};
use super::super::poseidon_merkle::{pack_ext, PoseidonMerkleTree};
use super::super::poseidon_transcript::PoseidonTranscript;
use super::types::{FriProofExtP, LayerOpeningExtP, QueryProofExtP};
use alloc::vec::Vec;

/// Prove `codeword` (an `Fp2` evaluation vector over `shift * {omega^i}`) has
/// degree below `2^k / 2^log_blowup`, committing with Poseidon and grinding
/// `grind_bits` of proof-of-work. `hasher` is shared by the Merkle nodes and the
/// transcript.
pub fn fri_prove_poseidon_ext(
    codeword: &[Fp2],
    shift: Fp,
    log_blowup: u32,
    n_queries: usize,
    grind_bits: u32,
    hasher: &Poseidon,
) -> FriProofExtP {
    let n = codeword.len();
    let log_n = n.trailing_zeros();
    let n_folds = (log_n - log_blowup) as usize;
    let base_omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();

    let mut transcript = PoseidonTranscript::new(hasher.clone());
    let mut current: Vec<Fp2> = codeword.to_vec();
    let mut layers: Vec<Vec<Fp2>> = Vec::with_capacity(n_folds);
    let mut trees: Vec<PoseidonMerkleTree> = Vec::with_capacity(n_folds);
    let mut roots: Vec<[Fp; RATE]> = Vec::with_capacity(n_folds);

    let mut omega = base_omega;
    let mut coset = shift;
    for _ in 0..n_folds {
        let leaves: Vec<[Fp; RATE]> = current.iter().map(|v| pack_ext(*v)).collect();
        let tree = PoseidonMerkleTree::commit(hasher, &leaves);
        let root = tree.root();
        transcript.absorb_digest(&root);
        let beta = transcript.challenge_fp2();
        let next = fold_ext(&current, beta, coset, omega, inv2);
        layers.push(current);
        trees.push(tree);
        roots.push(root);
        current = next;
        omega = omega.square();
        coset = coset.square();
    }

    let final_layer = current;
    for value in &final_layer {
        transcript.absorb(value.c0);
        transcript.absorb(value.c1);
    }
    let pow_nonce = transcript.grind(grind_bits);

    let mut queries: Vec<QueryProofExtP> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let q = transcript.challenge_index(n);
        let mut opened: Vec<LayerOpeningExtP> = Vec::with_capacity(n_folds);
        for m in 0..n_folds {
            let half = n >> (m + 1);
            let i = q % half;
            let layer = &layers[m];
            let tree = &trees[m];
            opened.push(LayerOpeningExtP {
                a: layer[i],
                a_path: tree.open(i),
                b: layer[i + half],
                b_path: tree.open(i + half),
            });
        }
        queries.push(QueryProofExtP { layers: opened });
    }

    FriProofExtP { roots, final_layer, queries, pow_nonce }
}
