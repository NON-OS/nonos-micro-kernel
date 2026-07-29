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

//! P-256 Elliptic Curve Diffie-Hellman (ECDH) key agreement.
//!
//! Implements SEC 1 §3.3.1 (ECDH Primitive) using the existing constant-time
//! P-256 scalar multiplication from `point::ProjectivePoint::mul`.

use super::{AffinePoint, PublicKey, Scalar, SecretKey};

/// Generate an ephemeral P-256 keypair for ECDH.
///
/// Returns `(secret_key [32 bytes], public_key [65 bytes uncompressed])`.
/// The public key format is `0x04 || x || y` as required by TLS key_share.
pub fn p256_ecdh_keypair() -> (SecretKey, PublicKey) {
    let mut sk = [0u8; 32];
    crate::crypto::rng::fill_random_bytes(&mut sk);

    loop {
        if let Some(scalar) = Scalar::from_bytes(&sk) {
            if !scalar.is_zero() {
                let point = AffinePoint::generator().to_projective().mul(&scalar).to_affine();
                if !point.infinity {
                    return (sk, point.to_uncompressed());
                }
            }
        }
        crate::crypto::rng::fill_random_bytes(&mut sk);
    }
}

/// Perform P-256 ECDH: compute shared secret from our secret key and peer's
/// uncompressed public key.
///
/// Returns the x-coordinate of `sk * peer_pub` as a 32-byte shared secret,
/// per SEC 1 §3.3.1 and RFC 8446 §4.2.8.2.
///
/// Validates that:
/// - The peer public key is a valid point on the P-256 curve
/// - The peer public key is not the point at infinity
/// - The resulting shared secret point is not the point at infinity
pub fn p256_ecdh(sk: &[u8; 32], peer_pub: &[u8; 65]) -> Option<[u8; 32]> {
    let scalar = Scalar::from_bytes(sk)?;
    if scalar.is_zero() {
        return None;
    }

    // AffinePoint::from_uncompressed validates the point is on the curve
    let peer_point = AffinePoint::from_uncompressed(peer_pub)?;
    if peer_point.infinity {
        return None;
    }

    let shared = peer_point.to_projective().mul(&scalar).to_affine();
    if shared.infinity {
        return None;
    }

    Some(shared.x.to_bytes())
}
