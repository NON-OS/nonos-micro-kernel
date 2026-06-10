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

use super::keccak::hash_pair;

pub fn build_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    let mut level: Vec<[u8; 32]> = leaves.to_vec();
    while level.len() > 1 {
        level = next_level(&level);
    }
    level[0]
}

pub fn build_proof(leaves: &[[u8; 32]], index: usize) -> Vec<[u8; 32]> {
    let mut proof = Vec::new();
    let mut level: Vec<[u8; 32]> = leaves.to_vec();
    let mut pos = index;
    while level.len() > 1 {
        let sibling = if pos % 2 == 0 { pos + 1 } else { pos - 1 };
        if sibling < level.len() {
            proof.push(level[sibling]);
        }
        pos /= 2;
        level = next_level(&level);
    }
    proof
}

pub fn process_proof(leaf: &[u8; 32], proof: &[[u8; 32]]) -> [u8; 32] {
    let mut node = *leaf;
    for sibling in proof {
        node = hash_pair(&node, sibling);
    }
    node
}

fn next_level(level: &[[u8; 32]]) -> Vec<[u8; 32]> {
    let mut up = Vec::with_capacity(level.len().div_ceil(2));
    for pair in level.chunks(2) {
        match pair {
            [a, b] => up.push(hash_pair(a, b)),
            [a] => up.push(*a),
            _ => unreachable!(),
        }
    }
    up
}
