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

//! Taking a public key and a signature apart, without branching out early.
//!
//! Every field can be malformed and each one is parsed into a usable stand-in
//! plus a flag, rather than returning on the first bad one. The flags are
//! ANDed together at the end, so a rejected signature runs the same sequence
//! of curve operations as an accepted one.
//!
//! Nothing here is secret: key, signature and hash are all public. The uniform
//! path is not hiding a key, it stops a verifier reporting through its timing
//! which field of a submitted signature was the bad one.

use crate::point::AffinePoint;
use crate::scalar::Scalar;
use crate::{PublicKey, Signature};

/// A signature and key parsed into curve values, with one flag saying whether
/// any of it was real. The values are safe to compute on either way.
pub(crate) struct Parts {
    pub point: AffinePoint,
    pub r: Scalar,
    pub s: Scalar,
    pub z: Scalar,
    pub valid: u64,
}

/// A scalar that is present and not zero, or ONE standing in for it. ONE is
/// chosen because it is invertible, so the caller's inversion has a defined
/// result on the reject path too.
fn nonzero(bytes: &[u8]) -> (Scalar, u64) {
    let fixed: &[u8; 32] = match bytes.try_into() {
        Ok(b) => b,
        Err(_) => return (Scalar::ONE, 0),
    };
    match Scalar::from_bytes(fixed) {
        Some(s) if !s.is_zero() => (s, 1),
        _ => (Scalar::ONE, 0),
    }
}

pub(crate) fn split(pk: &PublicKey, message_hash: &[u8; 32], sig: &Signature) -> Parts {
    let (point, key_valid) =
        match AffinePoint::from_uncompressed(pk) {
            Some(p) => (p, 1u64),
            None => (AffinePoint::identity(), 0u64),
        };
    let (r, r_valid) = nonzero(&sig[0..32]);
    let (s, s_valid) = nonzero(&sig[32..64]);

    /*
     * The hash is not required to be nonzero. A zero hash is a legitimate,
     * if astronomically unlikely, message digest, and ECDSA is defined for it.
     */
    let (z, z_valid) = match Scalar::from_bytes(message_hash) {
        Some(z) => (z, 1u64),
        None => (Scalar::ONE, 0u64),
    };

    Parts { point, r, s, z, valid: key_valid & r_valid & s_valid & z_valid }
}
