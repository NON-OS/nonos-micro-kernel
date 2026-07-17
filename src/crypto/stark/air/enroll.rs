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

//! Enrolling a set of capsule or kernel images into the policy root a spawn gate
//! trusts. Each image is measured to a leaf and the leaves are committed to a
//! Poseidon Merkle tree; the root is the value the kernel bakes and gates against.
//! This is what an enrollment tool computes and what the boot chain carries.

use super::super::field::Fp;
use super::super::poseidon_merkle::PoseidonMerkleTree;
use super::measure::measure_capsule;
use super::poseidon::{Poseidon, RATE};
use alloc::vec::Vec;

/// The policy root over `images`: measure each to a leaf, then commit the leaves.
pub fn enroll_policy_root(hasher: &Poseidon, images: &[&[u8]]) -> [Fp; RATE] {
    let leaves: Vec<[Fp; RATE]> = images.iter().map(|img| measure_capsule(hasher, img)).collect();
    PoseidonMerkleTree::commit(hasher, &leaves).root()
}
