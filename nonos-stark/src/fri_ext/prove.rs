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

//! The money-grade FRI prover. It lifts the base codeword into the extension,
//! commits each layer over `Fp2`, folds under an extension challenge, grinds a
//! proof-of-work nonce, then opens the sampled positions.

use super::super::field::{Fp, Fp2};
use super::super::fri::{fold_ext, root_of_unity};
use super::super::merkle::MerkleTree;
use super::super::transcript::Transcript;
use super::types::{FriProofExt, LayerOpeningExt, QueryProofExt};
use alloc::vec::Vec;

/// Prove `codeword` (an extension-field evaluation vector over the size-`2^k`
/// domain `shift * {omega^i}`) has degree below `2^k / 2^log_blowup`, with
/// extension-field folding and `grind_bits` of proof-of-work. `codeword.len()`
/// must be a power of two. A base codeword is lifted with `Fp2::from_base` by the
/// caller; the DEEP quotient is already `Fp2`, so it is passed directly.
pub fn fri_prove_ext(
    codeword: &[Fp2],
    shift: Fp,
    log_blowup: u32,
    n_queries: usize,
    grind_bits: u32,
) -> FriProofExt {
    let n = codeword.len();
    let log_n = n.trailing_zeros();
    let n_folds = (log_n - log_blowup) as usize;
    let base_omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();

    let mut transcript = Transcript::new(b"NONOS-STARK-FRI-EXT");
    let mut current: Vec<Fp2> = codeword.to_vec();
    let mut layers: Vec<Vec<Fp2>> = Vec::with_capacity(n_folds);
    let mut trees: Vec<MerkleTree> = Vec::with_capacity(n_folds);
    let mut roots: Vec<[u8; 32]> = Vec::with_capacity(n_folds);

    let mut omega = base_omega;
    let mut coset = shift;
    for _ in 0..n_folds {
        let tree = MerkleTree::commit_ext(&current);
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

    // Bind the final layer, then grind: the proof-of-work nonce is committed
    // before any query position is drawn, so it cannot be re-searched per query.
    let final_layer = current;
    for value in &final_layer {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let pow_nonce = transcript.grind(grind_bits);

    let mut queries: Vec<QueryProofExt> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let q = transcript.challenge_index(n);
        let mut opened: Vec<LayerOpeningExt> = Vec::with_capacity(n_folds);
        for m in 0..n_folds {
            let half = n >> (m + 1);
            let i = q % half;
            let layer = &layers[m];
            let tree = &trees[m];
            opened.push(LayerOpeningExt {
                a: layer[i],
                a_path: tree.open(i),
                b: layer[i + half],
                b_path: tree.open(i + half),
            });
        }
        queries.push(QueryProofExt { layers: opened });
    }

    FriProofExt { roots, final_layer, queries, pow_nonce }
}
