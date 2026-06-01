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

use super::field::FieldElement;
use super::point::AffinePoint;
use super::scalar::Scalar;
use super::{PublicKey, RecoverableSignature, SecretKey, Signature};

fn rfc6979_generate_k(sk: &[u8; 32], message_hash: &[u8; 32]) -> Option<Scalar> {
    let mut v = [0x01u8; 32];
    let mut k = [0x00u8; 32];

    let mut data = [0u8; 97];
    data[0..32].copy_from_slice(&v);
    data[32] = 0x00;
    data[33..65].copy_from_slice(sk);
    data[65..97].copy_from_slice(message_hash);
    k = crate::crypto::hmac_sha256(&k, &data);
    v = crate::crypto::hmac_sha256(&k, &v);

    data[0..32].copy_from_slice(&v);
    data[32] = 0x01;
    data[33..65].copy_from_slice(sk);
    data[65..97].copy_from_slice(message_hash);
    k = crate::crypto::hmac_sha256(&k, &data);
    v = crate::crypto::hmac_sha256(&k, &v);

    for _ in 0..256 {
        v = crate::crypto::hmac_sha256(&k, &v);
        if let Some(candidate) = Scalar::from_bytes(&v) {
            if !candidate.is_zero() {
                return Some(candidate);
            }
        }
        let mut retry_data = [0u8; 33];
        retry_data[0..32].copy_from_slice(&v);
        retry_data[32] = 0x00;
        k = crate::crypto::hmac_sha256(&k, &retry_data);
        v = crate::crypto::hmac_sha256(&k, &v);
    }
    None
}

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let mut sk = [0u8; 32];
    crate::crypto::rng::fill_random_bytes(&mut sk);

    loop {
        if let Some(scalar) = Scalar::from_bytes(&sk) {
            if !scalar.is_zero() {
                if let Some(pk) = public_key_from_secret(&sk) {
                    return (sk, pk);
                }
            }
        }
        crate::crypto::rng::fill_random_bytes(&mut sk);
    }
}

pub fn public_key_from_secret(sk: &SecretKey) -> Option<PublicKey> {
    let scalar = Scalar::from_bytes(sk)?;
    let point = AffinePoint::generator().to_projective().mul(&scalar).to_affine();
    Some(point.to_uncompressed())
}

// SECURITY: Constant-time signing with ct_select for high_s normalization
pub fn sign(sk: &SecretKey, message_hash: &[u8; 32]) -> Option<RecoverableSignature> {
    let d = Scalar::from_bytes(sk)?;
    let z = Scalar::from_bytes(message_hash)?;

    let k = rfc6979_generate_k(sk, message_hash)?;

    let r_point = AffinePoint::generator().to_projective().mul(&k).to_affine();
    if r_point.infinity {
        return None;
    }

    let r = Scalar::from_bytes(&r_point.x.to_bytes())?;
    if r.is_zero() {
        return None;
    }

    let k_inv = k.invert()?;
    let s = k_inv.mul(&z.add(&r.mul(&d)));

    if s.is_zero() {
        return None;
    }

    const HALF_N: [u64; 4] =
        [0xDFE92F46681B20A0, 0x5D576E7357A4501D, 0xFFFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF];
    let mut high_s = 0u64;
    let mut undecided = 1u64;
    for i in (0..4).rev() {
        let a = s.0[i];
        let b = HALF_N[i];
        let lt = (a ^ ((a ^ b) | (a.wrapping_sub(b) ^ b))) >> 63;
        let gt = (b ^ ((b ^ a) | (b.wrapping_sub(a) ^ a))) >> 63;
        high_s |= undecided & gt;
        undecided &= 1 ^ (gt | lt);
    }
    let s_negated = s.negate();
    let mask = 0u64.wrapping_sub(high_s);
    let s = Scalar::ct_select(mask, &s_negated, &s);

    let y_parity = if r_point.y.is_even() { 0u8 } else { 1u8 };
    let recovery_id = y_parity ^ (high_s as u8);

    Some(RecoverableSignature { r: r.to_bytes(), s: s.to_bytes(), recovery_id })
}

// SECURITY: Constant-time verification - performs all operations regardless of validity
// Uses error accumulation pattern to prevent timing side-channels
pub fn verify(pk: &PublicKey, message_hash: &[u8; 32], sig: &Signature) -> bool {
    // Track validity through all operations
    let mut valid: u64 = 1;

    // Parse public key - use identity point as dummy if invalid
    let point = match AffinePoint::from_uncompressed(pk.try_into().unwrap_or(&[0u8; 65])) {
        Some(p) => p,
        None => {
            valid = 0;
            AffinePoint::identity()
        }
    };

    // Parse r - use ONE as dummy if invalid
    let r_bytes: &[u8; 32] = sig[0..32].try_into().unwrap_or(&[0u8; 32]);
    let (r, r_valid) = match Scalar::from_bytes(r_bytes) {
        Some(r) if !r.is_zero() => (r, 1u64),
        Some(_) => (Scalar::ONE, 0u64),
        None => (Scalar::ONE, 0u64),
    };
    valid &= r_valid;

    // Parse s - use ONE as dummy if invalid
    let s_bytes: &[u8; 32] = sig[32..64].try_into().unwrap_or(&[0u8; 32]);
    let (s, s_valid) = match Scalar::from_bytes(s_bytes) {
        Some(s) if !s.is_zero() => (s, 1u64),
        Some(_) => (Scalar::ONE, 0u64),
        None => (Scalar::ONE, 0u64),
    };
    valid &= s_valid;

    // Parse z - use ONE as dummy if invalid
    let (z, z_valid) = match Scalar::from_bytes(message_hash) {
        Some(z) => (z, 1u64),
        None => (Scalar::ONE, 0u64),
    };
    valid &= z_valid;

    // Compute s_inv - use ONE as dummy if inversion fails
    let (s_inv, inv_valid) = match s.invert() {
        Some(inv) => (inv, 1u64),
        None => (Scalar::ONE, 0u64),
    };
    valid &= inv_valid;

    // Always compute u1, u2 regardless of validity
    let u1 = z.mul(&s_inv);
    let u2 = r.mul(&s_inv);

    let g = AffinePoint::generator().to_projective();
    let q = point.to_projective();

    let point_r = g.mul(&u1).add(&q.mul(&u2)).to_affine();

    // Check for point at infinity (constant-time using flag)
    let not_infinity = if point_r.infinity { 0u64 } else { 1u64 };
    valid &= not_infinity;

    // Always compute r from x-coordinate - use dummy value if needed
    let (computed_r, r_parse_valid) = match Scalar::from_bytes(&point_r.x.to_bytes()) {
        Some(r) => (r, 1u64),
        None => (Scalar::ONE, 0u64),
    };
    valid &= r_parse_valid;

    // Constant-time comparison
    let r_matches = if computed_r.ct_eq(&r) { 1u64 } else { 0u64 };
    valid &= r_matches;

    valid == 1
}

pub fn recover_public_key(
    message_hash: &[u8; 32],
    sig: &RecoverableSignature,
) -> Option<PublicKey> {
    let r = Scalar::from_bytes(&sig.r)?;
    let s = Scalar::from_bytes(&sig.s)?;
    let recovery_id = sig.recovery_id;

    if r.is_zero() || s.is_zero() || recovery_id > 3 {
        return None;
    }

    let z = Scalar::from_bytes(message_hash)?;

    let r_fe = FieldElement::from_bytes(&sig.r)?;
    let y_squared = r_fe.mul(&r_fe).mul(&r_fe).add(&FieldElement([7, 0, 0, 0]));
    let y = y_squared.sqrt()?;

    let y = if (recovery_id & 1 == 0) == y.is_even() { y } else { y.negate() };

    let r_point = AffinePoint { x: r_fe, y, infinity: false };

    let r_inv = r.invert()?;
    let u1 = z.negate().mul(&r_inv);
    let u2 = s.mul(&r_inv);

    let g = AffinePoint::generator().to_projective();
    let r_proj = r_point.to_projective();

    let pk_point = g.mul(&u1).add(&r_proj.mul(&u2)).to_affine();

    if pk_point.infinity {
        return None;
    }

    Some(pk_point.to_uncompressed())
}

pub fn eth_address(pk: &PublicKey) -> [u8; 20] {
    let hash = crate::crypto::sha3::keccak256(&pk[1..]);
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&hash[12..32]);
    addr
}

pub fn address_from_secret(sk: &SecretKey) -> [u8; 20] {
    match public_key_from_secret(sk) {
        Some(pk) => eth_address(&pk),
        None => [0u8; 20],
    }
}
