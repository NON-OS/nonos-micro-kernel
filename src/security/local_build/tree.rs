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

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::hash::blake3_hash;
use crate::crypto::zk_kernel::constants::{DOM_MERKLE_LEAF as DOM_LEAF, DOM_MERKLE_NODE as DOM_NODE};
use crate::security::capsule_attest::layout::POLICY_TREE_DEPTH;

// Fixed, not random: the machine has to be able to recompute the root it
// enrolled from the commitment alone.
const EMPTY_LEAF: [u8; 32] = [0u8; 32];

fn hash_leaf(value: &[u8; 32]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_LEAF.len() + 32);
    buf.extend_from_slice(DOM_LEAF);
    buf.extend_from_slice(value);
    blake3_hash(&buf)
}

fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_NODE.len() + 64);
    buf.extend_from_slice(DOM_NODE);
    buf.extend_from_slice(left);
    buf.extend_from_slice(right);
    blake3_hash(&buf)
}

/// Siblings for a leaf at index 0 of an otherwise empty tree: each level is
/// the level below, doubled.
pub(super) fn empty_siblings() -> Vec<[u8; 32]> {
    let mut out = Vec::with_capacity(POLICY_TREE_DEPTH);
    let mut level = hash_leaf(&EMPTY_LEAF);
    for _ in 0..POLICY_TREE_DEPTH {
        out.push(level);
        level = hash_node(&level, &level);
    }
    out
}

/// Folded the way `attest::merkle::fold_root` folds it, from the same domain
/// strings, or a proof minted here reaches a root nobody enrolled.
pub(super) fn root_for(commitment: &[u8; 32]) -> [u8; 32] {
    let mut current = hash_leaf(commitment);
    for sibling in empty_siblings() {
        current = hash_node(&current, &sibling);
    }
    current
}
