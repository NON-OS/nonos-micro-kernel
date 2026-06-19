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

use super::challenge::challenge;
use super::directions::directions_from_index;
use super::types::EnrolledSecretProof;
use super::super::field::FieldElement;
use super::super::pedersen::PedersenCommitment;
use crate::crypto::rng::get_random_bytes;

pub fn prove(
    secret: &[u8; 32],
    blinding: &[u8; 32],
    leaf_index: usize,
    siblings: &[[u8; 32]],
    root: &[u8; 32],
    ctx: &[u8],
) -> Option<EnrolledSecretProof> {
    let g = PedersenCommitment::generator_g()?;
    let h = PedersenCommitment::generator_h();

    let x = FieldElement::from_bytes(secret);
    let r = FieldElement::from_bytes(blinding);
    let commitment = PedersenCommitment::commit(&x.to_bytes(), &r.to_bytes()).commitment;

    let k_x = FieldElement::from_bytes(&get_random_bytes());
    let k_r = FieldElement::from_bytes(&get_random_bytes());
    let nonce_point = g.scalar_mul(&k_x.to_bytes()).add(&h.scalar_mul(&k_r.to_bytes())).compress();

    let directions = directions_from_index(leaf_index, siblings.len());
    let c = challenge(root, ctx, &commitment, &nonce_point, siblings, &directions);

    let z_x = k_x.add(&c.mul(&x)).to_bytes();
    let z_r = k_r.add(&c.mul(&r)).to_bytes();

    let mut sib = Vec::with_capacity(siblings.len());
    sib.extend_from_slice(siblings);

    Some(EnrolledSecretProof { commitment, nonce_point, z_x, z_r, siblings: sib, directions })
}
