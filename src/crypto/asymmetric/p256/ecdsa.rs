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

use super::{AffinePoint, PublicKey, Scalar, SecretKey, Signature};

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
    if scalar.is_zero() {
        return None;
    }
    let point = AffinePoint::generator().to_projective().mul(&scalar).to_affine();
    Some(point.to_uncompressed())
}

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

// SECURITY: Constant-time signing - uses ct_select for high_s normalization
pub fn sign(sk: &SecretKey, message_hash: &[u8; 32]) -> Option<Signature> {
    let d = Scalar::from_bytes(sk)?;
    let z = Scalar::from_bytes_reduce(message_hash);

    let k = rfc6979_generate_k(sk, message_hash)?;

    let r_point = AffinePoint::generator().to_projective().mul(&k).to_affine();
    if r_point.infinity {
        return None;
    }

    let r = Scalar::from_bytes_reduce(&r_point.x.to_bytes());
    if r.is_zero() {
        return None;
    }

    let k_inv = k.invert()?;
    let s = k_inv.mul(&z.add(&r.mul(&d)));

    if s.is_zero() {
        return None;
    }

    // SECURITY: Constant-time high-S normalization
    // Check if s > n/2 by looking at the high bit of the scalar
    let s_bytes = s.to_bytes();
    let high_s = (s_bytes[0] >> 7) & 1;
    let s_negated = s.negate();
    let mask = 0u64.wrapping_sub(high_s as u64);
    let s = Scalar::ct_select(mask, &s_negated, &s);

    let mut sig = [0u8; 64];
    sig[0..32].copy_from_slice(&r.to_bytes());
    sig[32..64].copy_from_slice(&s.to_bytes());

    Some(sig)
}

// SECURITY: Constant-time verification - performs all operations regardless of validity
// Uses error accumulation pattern to prevent timing side-channels
pub fn verify(pk: &PublicKey, message_hash: &[u8; 32], sig: &Signature) -> bool {
    // Track validity through all operations
    let mut valid: u64 = 1;

    // Parse public key - use identity point as dummy if invalid
    let point = match AffinePoint::from_uncompressed(pk) {
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
        Some(_) => (Scalar::ONE, 0u64), // r was zero
        None => (Scalar::ONE, 0u64),    // r was invalid
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

    let z = Scalar::from_bytes_reduce(message_hash);

    // Compute s_inv - use ONE as dummy if inversion fails (s was zero)
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
    let r_prime = g.mul(&u1).add(&q.mul(&u2)).to_affine();

    // Check for point at infinity (constant-time using flag)
    let not_infinity = if r_prime.infinity { 0u64 } else { 1u64 };
    valid &= not_infinity;

    // Always compute r from x-coordinate
    let computed_r = Scalar::from_bytes_reduce(&r_prime.x.to_bytes());

    let r_matches = if computed_r.ct_eq(&r) { 1u64 } else { 0u64 };
    valid &= r_matches;

    valid == 1
}

pub fn sign_message(sk: &SecretKey, message: &[u8]) -> Option<Signature> {
    let hash = crate::crypto::hash::sha256(message);
    sign(sk, &hash)
}

pub fn verify_message(pk: &PublicKey, message: &[u8], sig: &Signature) -> bool {
    let hash = crate::crypto::hash::sha256(message);
    verify(pk, &hash, sig)
}

#[derive(Clone)]
pub struct P256KeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

impl Drop for P256KeyPair {
    fn drop(&mut self) {
        // SAFETY: We use volatile writes to securely zeroize the secret key
        // to prevent data remanence attacks. The compiler_fence ensures the
        // writes are not reordered or optimized away.
        for byte in self.secret_key.iter_mut() {
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

impl P256KeyPair {
    pub fn generate() -> Self {
        let (secret_key, public_key) = generate_keypair();
        Self { secret_key, public_key }
    }

    pub fn sign(&self, message: &[u8]) -> Option<Signature> {
        sign_message(&self.secret_key, message)
    }

    pub fn verify(&self, message: &[u8], sig: &Signature) -> bool {
        verify_message(&self.public_key, message, sig)
    }
}
