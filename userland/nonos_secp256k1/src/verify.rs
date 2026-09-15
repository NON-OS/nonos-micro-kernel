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

//! ECDSA verification.
//!
//! The check is the textbook one: recompute R from the signature and the key,
//! and accept when its x coordinate is the r that was sent. What is not
//! textbook is that it runs the same way on a bad signature as on a good one,
//! which [`crate::verify_parts`] explains.

use crate::point::AffinePoint;
use crate::scalar::Scalar;
use crate::verify_parts::split;
use crate::{PublicKey, Signature};

pub fn verify(pk: &PublicKey, message_hash: &[u8; 32], sig: &Signature) -> bool {
    let parts = split(pk, message_hash, sig);
    let mut valid = parts.valid;

    /*
     * ONE is substituted for a rejected s, so this always succeeds in
     * practice. The flag is kept rather than unwrapped because an inversion
     * that fails must not be able to reach the comparison below.
     */
    let (s_inv, inv_valid) = match parts.s.invert() {
        Some(inv) => (inv, 1u64),
        None => (Scalar::ONE, 0u64),
    };
    valid &= inv_valid;

    let u1 = parts.z.mul(&s_inv);
    let u2 = parts.r.mul(&s_inv);

    let g = AffinePoint::generator().to_projective();
    let point_r = g.mul(&u1).add(&parts.point.to_projective().mul(&u2)).to_affine();
    valid &= if point_r.infinity { 0 } else { 1 };

    /*
     * R's x is a field element and r is a scalar; an x at or above the group
     * order has no scalar and cannot be the r that was sent, so failing to
     * parse it is a rejection rather than an error.
     */
    let (computed_r, parsed) = match Scalar::from_bytes(&point_r.x.to_bytes()) {
        Some(r) => (r, 1u64),
        None => (Scalar::ONE, 0u64),
    };
    valid &= parsed;
    valid &= if computed_r.ct_eq(&parts.r) { 1 } else { 0 };

    valid == 1
}
