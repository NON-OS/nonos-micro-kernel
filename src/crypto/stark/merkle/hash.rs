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

//! Domain-separated leaf and node hashing for the Merkle commitment. BLAKE3 is
//! the hash, already proven correct against its reference vectors elsewhere in
//! the tree, so the commitment's security rests on a checked primitive.

use super::super::field::Fp;
use crate::crypto::hash::blake3_hash;
use alloc::vec::Vec;

const DOM_LEAF: &[u8] = b"NONOS-STARK-MERKLE-LEAF";
const DOM_NODE: &[u8] = b"NONOS-STARK-MERKLE-NODE";

/// Hash a field element into a leaf digest, domain-separated from node hashing
/// so a leaf can never be reinterpreted as an internal node.
pub(super) fn hash_leaf(leaf: Fp) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_LEAF.len() + 8);
    buf.extend_from_slice(DOM_LEAF);
    buf.extend_from_slice(&leaf.value().to_le_bytes());
    blake3_hash(&buf)
}

/// Hash two child digests into their parent, with a distinct domain tag.
pub(super) fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_NODE.len() + 64);
    buf.extend_from_slice(DOM_NODE);
    buf.extend_from_slice(left);
    buf.extend_from_slice(right);
    blake3_hash(&buf)
}
