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

//! Verifying a Poseidon Merkle path. Recompute the root from a leaf and its
//! siblings with the same two-to-one compression, and compare. Every step is a
//! field operation, which is exactly the property a recursive verifier needs:
//! this check can be re-expressed as AIR constraints and proven inside a STARK.

use super::super::air::{Poseidon, RATE};
use super::super::field::Fp;

/// Recompute the root implied by `leaf` at `index` under `path` and return
/// whether it equals `root`. A path of the wrong length, or a tampered leaf,
/// sibling, or root, all yield `false`.
pub fn verify_path(
    hasher: &Poseidon,
    root: &[Fp; RATE],
    index: usize,
    leaf: [Fp; RATE],
    path: &[[Fp; RATE]],
) -> bool {
    let mut node = leaf;
    let mut idx = index;
    for sibling in path {
        node = if idx & 1 == 0 {
            hasher.compress(&node, sibling)
        } else {
            hasher.compress(sibling, &node)
        };
        idx >>= 1;
    }
    idx == 0 && node == *root
}
