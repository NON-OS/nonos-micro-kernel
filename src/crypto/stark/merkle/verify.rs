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

//! The verifying side of the Merkle commitment: recompute the root from a leaf
//! and its authentication path and compare. This is what the STARK verifier
//! calls; it takes only untrusted inputs and never panics.

use super::super::field::Fp;
use super::hash::{hash_leaf, hash_node};

/// Recompute the root implied by `leaf` sitting at `index` under `path`, and
/// return whether it equals the committed `root`. A path of the wrong length,
/// a tampered leaf, sibling, or root all yield `false`.
pub fn verify_path(root: &[u8; 32], index: usize, leaf: Fp, path: &[[u8; 32]]) -> bool {
    let mut node = hash_leaf(leaf);
    let mut idx = index;
    for sibling in path {
        node = if idx & 1 == 0 {
            hash_node(&node, sibling)
        } else {
            hash_node(sibling, &node)
        };
        idx >>= 1;
    }
    // After consuming the whole path the index must have reached the root row.
    idx == 0 && node == *root
}
