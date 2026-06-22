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

use super::super::pedersen::PedersenCommitment;
use super::super::utils::constant_time_eq;
use super::challenge::challenge;
use super::merkle::fold_root;
use super::types::EnrolledSecretProof;
use crate::crypto::curve25519::EdwardsPoint;

pub fn verify(proof: &EnrolledSecretProof, root: &[u8; 32], ctx: &[u8]) -> bool {
    if proof.siblings.len() != proof.directions.len() {
        return false;
    }
    let g = match PedersenCommitment::generator_g() {
        Some(p) => p,
        None => return false,
    };
    let h = PedersenCommitment::generator_h();

    let c_point = EdwardsPoint::decompress(&proof.commitment);
    let a_point = EdwardsPoint::decompress(&proof.nonce_point);

    let mut valid: u8 = 1;
    valid &= c_point.is_some() as u8;
    valid &= a_point.is_some() as u8;
    let c_pt = c_point.unwrap_or_else(EdwardsPoint::identity);
    let a_pt = a_point.unwrap_or_else(EdwardsPoint::identity);
    valid &= (!c_pt.double().double().double().is_identity()) as u8;
    valid &= (!a_pt.double().double().double().is_identity()) as u8;

    let computed = fold_root(&proof.commitment, &proof.siblings, &proof.directions);
    valid &= constant_time_eq(&computed, root) as u8;

    let c = challenge(
        root,
        ctx,
        &proof.commitment,
        &proof.nonce_point,
        &proof.siblings,
        &proof.directions,
    );
    let lhs = g.scalar_mul(&proof.z_x).add(&h.scalar_mul(&proof.z_r));
    let rhs = a_pt.add(&c_pt.scalar_mul(&c.to_bytes()));
    valid &= constant_time_eq(&lhs.compress(), &rhs.compress()) as u8;

    valid == 1
}
