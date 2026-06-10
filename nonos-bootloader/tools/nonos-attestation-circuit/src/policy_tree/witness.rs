// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use ark_crypto_primitives::sponge::Absorb;
use ark_ff::PrimeField;

use super::constants::{POLICY_TREE_DEPTH, POLICY_TREE_LEAVES};
use super::hash_leaf::hash_leaf;
use super::hash_pair::hash_pair;
use super::types::PolicyWitness;

pub fn witness<F: PrimeField + Absorb>(
    entries: &[(F, F, u64)],
    mut index: usize,
) -> Result<PolicyWitness<F>, String> {
    if entries.is_empty() || entries.len() > POLICY_TREE_LEAVES || index >= entries.len() {
        return Err("policy tree bounds".into());
    }
    let empty = hash_leaf(F::from(0u64), F::from(0u64), 0)?;
    let mut layer = Vec::with_capacity(POLICY_TREE_LEAVES);
    for (hi, lo, caps) in entries.iter().copied() {
        layer.push(hash_leaf(hi, lo, caps)?);
    }
    layer.resize(POLICY_TREE_LEAVES, empty);
    let mut siblings = Vec::with_capacity(POLICY_TREE_DEPTH);
    let mut dirs = Vec::with_capacity(POLICY_TREE_DEPTH);
    for _ in 0..POLICY_TREE_DEPTH {
        let right = index & 1 == 1;
        let sibling = if right { index - 1 } else { index + 1 };
        siblings.push(layer[sibling]);
        dirs.push(right);
        let mut next = Vec::with_capacity(layer.len() / 2);
        for pair in layer.chunks_exact(2) {
            next.push(hash_pair(pair[0], pair[1])?);
        }
        layer = next;
        index /= 2;
    }
    Ok(PolicyWitness { root: layer[0], siblings, dirs })
}
