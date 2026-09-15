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

//! Signing.
//!
//! Deterministic throughout: the nonce comes from the key and the message,
//! and s is forced low, so one key signing one hash always produces the same
//! sixty-five bytes. Nothing here reads a clock or an entropy pool.

use crate::low_s::normalise;
use crate::point::AffinePoint;
use crate::rfc6979::rfc6979_generate_k;
use crate::scalar::Scalar;
use crate::{RecoverableSignature, SecretKey};

/// `None` rather than a signature for anything that is not a signable input.
///
/// The zero key is refused before it reaches the nonce derivation.
/// `Scalar::from_bytes` only checks that a value is below the group order, and
/// zero is, so it used to sign: the resulting signature verified against the
/// encoded point at infinity, which is what `public_key_from_secret` handed
/// back for the same key. Both ends agreed and neither had a private key.
pub fn sign(sk: &SecretKey, message_hash: &[u8; 32]) -> Option<RecoverableSignature> {
    let d = Scalar::from_bytes(sk)?;
    if d.is_zero() {
        return None;
    }
    let z = Scalar::from_bytes(message_hash)?;
    let k = rfc6979_generate_k(sk, message_hash)?;

    let r_point = AffinePoint::generator().to_projective().mul(&k).to_affine();
    if r_point.infinity {
        return None;
    }

    /*
     * R's x is reduced from the field into the scalar order here. The two
     * moduli are close enough that this is almost always the identity, and an
     * r of zero is the case where it was not and landed on the order itself.
     */
    let r = Scalar::from_bytes(&r_point.x.to_bytes())?;
    if r.is_zero() {
        return None;
    }

    let k_inv = k.invert()?;
    let s = k_inv.mul(&z.add(&r.mul(&d)));
    if s.is_zero() {
        return None;
    }

    let y_parity = if r_point.y.is_even() { 0 } else { 1 };
    let (s, recovery_id) = normalise(s, y_parity);
    Some(RecoverableSignature { r: r.to_bytes(), s: s.to_bytes(), recovery_id })
}
