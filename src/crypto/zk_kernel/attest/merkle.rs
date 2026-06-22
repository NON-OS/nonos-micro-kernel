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

use super::super::constants::{DOM_MERKLE_LEAF, DOM_MERKLE_NODE};
use crate::crypto::hash::blake3_hash;

pub(super) fn fold_root(
    commitment: &[u8; 32],
    siblings: &[[u8; 32]],
    directions: &[u8],
) -> [u8; 32] {
    let mut leaf = Vec::with_capacity(DOM_MERKLE_LEAF.len() + 32);
    leaf.extend_from_slice(DOM_MERKLE_LEAF);
    leaf.extend_from_slice(commitment);
    let mut current = blake3_hash(&leaf);
    for (sibling, direction) in siblings.iter().zip(directions.iter()) {
        let mut buf = Vec::with_capacity(DOM_MERKLE_NODE.len() + 64);
        buf.extend_from_slice(DOM_MERKLE_NODE);
        if *direction == 0 {
            buf.extend_from_slice(&current);
            buf.extend_from_slice(sibling);
        } else {
            buf.extend_from_slice(sibling);
            buf.extend_from_slice(&current);
        }
        current = blake3_hash(&buf);
    }
    current
}
