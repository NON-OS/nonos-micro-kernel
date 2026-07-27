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

//! Building the Merkle tree and opening authentication paths. This is the
//! committing side; a verifier only needs `super::verify`.

use super::super::field::{Fp, Fp2};
use super::hash::{hash_leaf, hash_leaf_ext, hash_leaf_wide, hash_leaf_wide_periodic, hash_node};
use alloc::vec::Vec;

/// A committed binary Merkle tree over field-element leaves. The leaf count is
/// padded up to a power of two, matching a STARK evaluation domain.
pub struct MerkleTree {
    layers: Vec<Vec<[u8; 32]>>,
    root: [u8; 32],
}

impl MerkleTree {
    /// Commit to a sequence of leaves, returning the tree. An empty input
    /// commits to a single zero leaf so the root is always defined.
    pub fn commit(leaves: &[Fp]) -> MerkleTree {
        let mut level: Vec<[u8; 32]> = crate::par::map_slice(leaves, |l| hash_leaf(*l));
        if level.is_empty() {
            level.push(hash_leaf(Fp::ZERO));
        }
        while !level.len().is_power_of_two() {
            level.push(hash_leaf(Fp::ZERO));
        }
        Self::build(level)
    }

    /// Commit to extension-field leaves, the folded FRI layers past the first.
    /// Same tree structure and node hashing as `commit`; only the leaf hash
    /// differs, so paths open identically via `verify_path_ext`.
    pub fn commit_ext(leaves: &[Fp2]) -> MerkleTree {
        let mut level: Vec<[u8; 32]> = crate::par::map_slice(leaves, |l| hash_leaf_ext(*l));
        if level.is_empty() {
            level.push(hash_leaf_ext(Fp2::ZERO));
        }
        while !level.len().is_power_of_two() {
            level.push(hash_leaf_ext(Fp2::ZERO));
        }
        Self::build(level)
    }

    /// Commit to a trace row-wise: leaf `i` hashes every column's value at row `i`
    /// as one wide leaf. One tree and one root cover the whole trace, so a query
    /// authenticates all columns of a row with a single path instead of one path
    /// per column. `columns` are the extended columns, all the same length.
    pub fn commit_wide(columns: &[Vec<Fp>]) -> MerkleTree {
        Self::commit_rows(columns, hash_leaf_wide)
    }

    /// Commit the preprocessed periodic columns row-wise under their own leaf
    /// domain: the structural commitment a verifier bakes as a constant root
    /// and authenticates per-query openings against.
    pub fn commit_wide_periodic(columns: &[Vec<Fp>]) -> MerkleTree {
        Self::commit_rows(columns, hash_leaf_wide_periodic)
    }

    fn commit_rows(columns: &[Vec<Fp>], leaf: fn(&[Fp]) -> [u8; 32]) -> MerkleTree {
        let width = columns.len();
        let n = columns.first().map(Vec::len).unwrap_or(0);
        // Each row leaf is an independent hash of the columns at that index, so
        // the leaf layer parallelizes cleanly; the indexed map keeps row order.
        let mut level: Vec<[u8; 32]> = crate::par::map_index(n, |i| {
            let row: Vec<Fp> = columns.iter().map(|col| col[i]).collect();
            leaf(&row)
        });
        let pad = alloc::vec![Fp::ZERO; width];
        if level.is_empty() {
            level.push(leaf(&pad));
        }
        while !level.len().is_power_of_two() {
            level.push(leaf(&pad));
        }
        Self::build(level)
    }

    /// Build a tree from precomputed leaf digests, padding to a power of two
    /// with `pad_leaf`. This is the seam a parallel committer uses: it hashes
    /// the wide leaves itself (row order preserved) and hands the digests here,
    /// so the tree above them is built by the one shared `build` and the root
    /// is identical to the serial `commit_wide_periodic` on the same rows.
    pub fn from_leaf_digests(mut level: Vec<[u8; 32]>, pad_leaf: [u8; 32]) -> MerkleTree {
        if level.is_empty() {
            level.push(pad_leaf);
        }
        while !level.len().is_power_of_two() {
            level.push(pad_leaf);
        }
        Self::build(level)
    }

    /// Build the tree above a power-of-two leaf-digest level. Shared by both
    /// commit paths so node hashing has a single implementation.
    fn build(mut level: Vec<[u8; 32]>) -> MerkleTree {
        let mut layers = Vec::new();
        layers.push(level.clone());
        while level.len() > 1 {
            let mut next = Vec::with_capacity(level.len() / 2);
            let mut i = 0;
            while i + 1 < level.len() {
                next.push(hash_node(&level[i], &level[i + 1]));
                i += 2;
            }
            level = next;
            layers.push(level.clone());
        }

        let root = layers.last().and_then(|top| top.first()).copied().unwrap_or([0u8; 32]);
        MerkleTree { layers, root }
    }

    /// The commitment root.
    pub fn root(&self) -> [u8; 32] {
        self.root
    }

    /// The number of padded leaves.
    pub fn len(&self) -> usize {
        self.layers.first().map(Vec::len).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The authentication path for a leaf: the sibling digest at each level from
    /// the leaves up to just below the root. Returns an empty path if the index
    /// is out of range.
    pub fn open(&self, index: usize) -> Vec<[u8; 32]> {
        let mut path = Vec::new();
        if index >= self.len() {
            return path;
        }
        let mut idx = index;
        for level in &self.layers {
            if level.len() <= 1 {
                break;
            }
            let sibling = idx ^ 1;
            if let Some(node) = level.get(sibling) {
                path.push(*node);
            }
            idx >>= 1;
        }
        path
    }
}
