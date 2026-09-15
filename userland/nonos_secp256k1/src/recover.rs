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

//! Recovering the signer's public key from a signature.
//!
//! Ethereum sends a signature as `r || s || v` and no public key, so the
//! address of the sender is whatever key the signature recovers to. That makes
//! recovery part of verification rather than a convenience: get it wrong and a
//! signature attributes to the wrong account.

use crate::point::AffinePoint;
use crate::recover_point::r_point;
use crate::scalar::Scalar;
use crate::{PublicKey, RecoverableSignature};

/// `None` when the signature recovers to nothing, which includes every input
/// this implementation does not handle rather than a key it cannot vouch for.
///
/// Recovery ids 2 and 3 say the x coordinate of R was `r + n`, the case where
/// the nonce's x fell in the narrow window above the group order. This does
/// not reduce that case and refuses it. It cannot arise from [`crate::sign`],
/// which reports only 0 and 1, and a correct signer produces one with about
/// probability 2^-128. Computing on the unreduced r instead, which is what
/// this code did before, returns a key that never signed anything, and a
/// caller comparing it against an expected address gets a mismatch it cannot
/// tell apart from a forgery.
pub fn recover_public_key(
    message_hash: &[u8; 32],
    sig: &RecoverableSignature,
) -> Option<PublicKey> {
    if sig.recovery_id > 1 {
        return None;
    }
    let r = Scalar::from_bytes(&sig.r)?;
    let s = Scalar::from_bytes(&sig.s)?;
    if r.is_zero() || s.is_zero() {
        return None;
    }
    let z = Scalar::from_bytes(message_hash)?;
    let point = r_point(&sig.r, sig.recovery_id & 1 == 1)?;

    let r_inv = r.invert()?;
    let u1 = z.negate().mul(&r_inv);
    let u2 = s.mul(&r_inv);

    let g = AffinePoint::generator().to_projective();
    let key = g.mul(&u1).add(&point.to_projective().mul(&u2)).to_affine();
    if key.infinity {
        return None;
    }
    Some(key.to_uncompressed())
}
