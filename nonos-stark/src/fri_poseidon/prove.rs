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

//! The Poseidon-committed FRI prover: commit each folded layer with a Poseidon
//! Merkle tree, drive the challenges from a Poseidon transcript, and open the
//! sampled positions. The commitment and the transcript are algebraic, so the
//! proof can be verified inside a STARK.

use super::super::air::{Poseidon, RATE};
use super::super::field::Fp;
use super::super::fri::root_of_unity;
use super::super::poseidon_merkle::PoseidonMerkleTree;
use super::super::poseidon_transcript::PoseidonTranscript;
use super::fold::fold_layer;
use super::types::{FriProof, LayerOpening, QueryProof};
use alloc::vec::Vec;

/// A codeword value embedded as a rate-sized Merkle leaf.
pub(super) fn leaf(value: Fp) -> [Fp; RATE] {
    let mut d = [Fp::ZERO; RATE];
    d[0] = value;
    d
}

/// Prove `codeword` is low degree, committing with Poseidon. `hasher` is the
/// Poseidon used for both the Merkle nodes and the transcript.
pub fn fri_prove(
    codeword: &[Fp],
    shift: Fp,
    log_blowup: u32,
    n_queries: usize,
    hasher: &Poseidon,
) -> FriProof {
    let n = codeword.len();
    let log_n = n.trailing_zeros();
    let n_folds = (log_n - log_blowup) as usize;
    let base_omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();

    let mut transcript = PoseidonTranscript::new(hasher.clone());
    let mut layers: Vec<Vec<Fp>> = Vec::with_capacity(n_folds);
    let mut trees: Vec<PoseidonMerkleTree> = Vec::with_capacity(n_folds);
    let mut roots: Vec<[Fp; RATE]> = Vec::with_capacity(n_folds);

    let mut current = codeword.to_vec();
    let mut omega = base_omega;
    let mut coset = shift;
    for _ in 0..n_folds {
        let leaves: Vec<[Fp; RATE]> = current.iter().map(|v| leaf(*v)).collect();
        let tree = PoseidonMerkleTree::commit(hasher, &leaves);
        let root = tree.root();
        transcript.absorb_digest(&root);
        let beta = transcript.challenge();
        let next = fold_layer(&current, beta, coset, omega, inv2);
        layers.push(current);
        trees.push(tree);
        roots.push(root);
        current = next;
        omega = omega.square();
        coset = coset.square();
    }

    let final_layer = current;
    for value in &final_layer {
        transcript.absorb(*value);
    }

    let mut queries: Vec<QueryProof> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let q = transcript.challenge_index(n);
        let mut opened: Vec<LayerOpening> = Vec::with_capacity(n_folds);
        for (layer, tree) in layers.iter().zip(trees.iter()) {
            let half = layer.len() / 2;
            let i = q % half;
            opened.push(LayerOpening {
                a: layer[i],
                a_path: tree.open(i),
                b: layer[i + half],
                b_path: tree.open(i + half),
            });
        }
        queries.push(QueryProof { layers: opened });
    }

    FriProof { roots, final_layer, queries }
}
