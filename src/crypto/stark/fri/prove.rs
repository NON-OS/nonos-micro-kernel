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

//! The FRI prover: commit each layer, fold under a Fiat-Shamir challenge, then
//! open the sampled query positions.

use super::super::field::Fp;
use super::super::merkle::MerkleTree;
use super::super::transcript::Transcript;
use super::domain::root_of_unity;
use super::fold::fold_layer;
use super::types::{FriProof, LayerOpening, QueryProof};
use alloc::vec::Vec;

/// Prove that `codeword`, the evaluations of a polynomial over the size-`2^k`
/// subgroup domain, has degree below `2^k / 2^log_blowup`. Folds `k - log_blowup`
/// times down to a constant layer of `2^log_blowup` values, then answers
/// `n_queries` sampled positions. The domain is `shift * {omega^i}`; pass
/// `shift = Fp::ONE` for the raw subgroup. `codeword.len()` must be a power of two.
pub fn fri_prove(codeword: &[Fp], shift: Fp, log_blowup: u32, n_queries: usize) -> FriProof {
    let n = codeword.len();
    let log_n = n.trailing_zeros();
    let n_folds = (log_n - log_blowup) as usize;
    let base_omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();

    let mut transcript = Transcript::new(b"NONOS-STARK-FRI");
    let mut layers: Vec<Vec<Fp>> = Vec::with_capacity(n_folds);
    let mut trees: Vec<MerkleTree> = Vec::with_capacity(n_folds);
    let mut roots: Vec<[u8; 32]> = Vec::with_capacity(n_folds);

    let mut current = codeword.to_vec();
    let mut omega = base_omega;
    let mut coset = shift;
    for _ in 0..n_folds {
        let tree = MerkleTree::commit(&current);
        let root = tree.root();
        transcript.absorb_digest(&root);
        let beta = transcript.challenge_fp();
        let next = fold_layer(&current, beta, coset, omega, inv2);
        layers.push(current);
        trees.push(tree);
        roots.push(root);
        current = next;
        omega = omega.square();
        coset = coset.square();
    }

    // The final layer is small and constant for a genuine low-degree input; it
    // is sent in full and bound into the transcript before the queries.
    let final_layer = current;
    for value in &final_layer {
        transcript.absorb_fp(*value);
    }

    let mut queries: Vec<QueryProof> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let q = transcript.challenge_index(n);
        let mut opened: Vec<LayerOpening> = Vec::with_capacity(n_folds);
        for m in 0..n_folds {
            let half = n >> (m + 1);
            let i = q % half;
            let layer = &layers[m];
            let tree = &trees[m];
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
