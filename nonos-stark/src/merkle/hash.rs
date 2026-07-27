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

//! Domain-separated leaf and node hashing for the Merkle commitment. Keccak256 is
//! the hash, already the EVM-native hash, so the on-chain verifier recomputes leaves and nodes with a
//! single native opcode instead of a costly in-Solidity hash.

use super::super::field::{Fp, Fp2};
use crate::hash::keccak256;
use alloc::vec::Vec;

const DOM_LEAF: &[u8] = b"NONOS-STARK-MERKLE-LEAF";
const DOM_LEAF_EXT: &[u8] = b"NONOS-STARK-MERKLE-LEAF-EXT";
const DOM_LEAF_WIDE: &[u8] = b"NONOS-STARK-MERKLE-LEAF-WIDE";
const DOM_LEAF_PERIODIC: &[u8] = b"NONOS-STARK-PERIODIC-WIDE";
const DOM_NODE: &[u8] = b"NONOS-STARK-MERKLE-NODE";

/// Hash a field element into a leaf digest, domain-separated from node hashing
/// so a leaf can never be reinterpreted as an internal node.
pub(super) fn hash_leaf(leaf: Fp) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_LEAF.len() + 8);
    buf.extend_from_slice(DOM_LEAF);
    buf.extend_from_slice(&leaf.value().to_le_bytes());
    keccak256(&buf)
}

/// Hash an extension-field element into a leaf digest: both lanes under a distinct
/// domain, so a folded FRI layer commits like a base layer but can never be
/// confused with one, an internal node, or a differently-shaped leaf.
pub(super) fn hash_leaf_ext(leaf: Fp2) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_LEAF_EXT.len() + 16);
    buf.extend_from_slice(DOM_LEAF_EXT);
    buf.extend_from_slice(&leaf.c0.value().to_le_bytes());
    buf.extend_from_slice(&leaf.c1.value().to_le_bytes());
    keccak256(&buf)
}

/// Hash a whole trace row into one leaf: every column's canonical value as an
/// 8-byte little-endian integer, tightly packed in column order, under a domain
/// distinct from the single-value leaf so a wide leaf can never be confused with a
/// base leaf, an extension leaf, or an internal node. This is the commitment shape
/// the on-chain verifier recomputes once per query instead of one path per column.
pub(super) fn hash_leaf_wide(row: &[Fp]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_LEAF_WIDE.len() + row.len() * 8);
    buf.extend_from_slice(DOM_LEAF_WIDE);
    for v in row {
        buf.extend_from_slice(&v.value().to_le_bytes());
    }
    keccak256(&buf)
}

/// Hash a row of preprocessed periodic-column values into one leaf: identical
/// packing to the wide trace leaf under its own domain, so the structural
/// periodic commitment a verifier bakes as a constant can never be read as a
/// trace leaf.
pub fn hash_leaf_wide_periodic(row: &[Fp]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_LEAF_PERIODIC.len() + row.len() * 8);
    buf.extend_from_slice(DOM_LEAF_PERIODIC);
    for v in row {
        buf.extend_from_slice(&v.value().to_le_bytes());
    }
    keccak256(&buf)
}

/// Hash two child digests into their parent, with a distinct domain tag.
pub(super) fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(DOM_NODE.len() + 64);
    buf.extend_from_slice(DOM_NODE);
    buf.extend_from_slice(left);
    buf.extend_from_slice(right);
    keccak256(&buf)
}
