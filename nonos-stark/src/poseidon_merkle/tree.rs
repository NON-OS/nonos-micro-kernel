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

//! Building the Poseidon Merkle tree and opening authentication paths.

use super::super::air::{Poseidon, RATE};
use super::super::field::Fp;
use alloc::vec::Vec;

/// A committed binary Merkle tree over `RATE`-element digests, whose node hash is
/// the Poseidon two-to-one compression. The leaf count is padded to a power of
/// two.
pub struct PoseidonMerkleTree {
    layers: Vec<Vec<[Fp; RATE]>>,
    root: [Fp; RATE],
}

impl PoseidonMerkleTree {
    /// Commit to a sequence of digest leaves under `hasher`. An empty input
    /// commits to a single zero leaf so the root is always defined.
    pub fn commit(hasher: &Poseidon, leaves: &[[Fp; RATE]]) -> PoseidonMerkleTree {
        let mut level: Vec<[Fp; RATE]> = leaves.to_vec();
        if level.is_empty() {
            level.push([Fp::ZERO; RATE]);
        }
        while !level.len().is_power_of_two() {
            level.push([Fp::ZERO; RATE]);
        }

        let mut layers = Vec::new();
        layers.push(level.clone());
        while level.len() > 1 {
            let mut next = Vec::with_capacity(level.len() / 2);
            let mut i = 0;
            while i + 1 < level.len() {
                next.push(hasher.compress(&level[i], &level[i + 1]));
                i += 2;
            }
            level = next;
            layers.push(level.clone());
        }

        let root = layers.last().and_then(|top| top.first()).copied().unwrap_or([Fp::ZERO; RATE]);
        PoseidonMerkleTree { layers, root }
    }

    /// The commitment root.
    pub fn root(&self) -> [Fp; RATE] {
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
    /// the leaves up to just below the root. Empty if the index is out of range.
    pub fn open(&self, index: usize) -> Vec<[Fp; RATE]> {
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
