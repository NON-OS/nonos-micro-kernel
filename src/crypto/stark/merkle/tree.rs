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

use super::super::field::Fp;
use super::hash::{hash_leaf, hash_node};
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
        let mut level: Vec<[u8; 32]> = leaves.iter().map(|l| hash_leaf(*l)).collect();
        if level.is_empty() {
            level.push(hash_leaf(Fp::ZERO));
        }
        while !level.len().is_power_of_two() {
            level.push(hash_leaf(Fp::ZERO));
        }

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
