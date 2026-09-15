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

use super::keys::{rsa_private_operation, rsa_public_operation, RsaPrivateKey, RsaPublicKey};
use super::mgf1;
use crate::crypto::entropy::get_entropy;
use crate::crypto::hash::sha256;
use crate::crypto::hash::sha384::sha384;
use crate::crypto::util::bigint::BigUint;
use alloc::vec;
use alloc::vec::Vec;

pub fn sign_pss(msg: &[u8], key: &RsaPrivateKey) -> Result<Vec<u8>, &'static str> {
    let hash = sha256(msg);
    let salt = get_entropy(32);

    let em_bits = key.bits - 1;
    let em_len = em_bits.div_ceil(8);
    let hash_len = 32;
    let salt_len = 32;

    if em_len < hash_len + salt_len + 2 {
        return Err("Key too small for PSS");
    }

    let mut m_prime = vec![0u8; 8];
    m_prime.extend_from_slice(&hash);
    m_prime.extend_from_slice(&salt);

    let h = sha256(&m_prime);

    let ps_len = em_len - salt_len - hash_len - 2;
    let mut db = vec![0u8; ps_len];
    db.push(0x01);
    db.extend_from_slice(&salt);

    let db_mask = mgf1(&h, db.len());
    let mut masked_db: Vec<u8> = db.iter().zip(db_mask.iter()).map(|(a, b)| a ^ b).collect();

    let top_bits = 8 * em_len - em_bits;
    if top_bits > 0 {
        masked_db[0] &= 0xFFu8 >> top_bits;
    }

    let mut em = masked_db;
    em.extend_from_slice(&h);
    em.push(0xbc);

    let message = BigUint::from_bytes_be(&em);
    match rsa_private_operation(&message, key) {
        Ok(sig) => {
            let mut result = sig.to_bytes_be();
            while result.len() < key.bits / 8 {
                result.insert(0, 0);
            }
            Ok(result)
        }
        Err(_) => Err("RSA signing failed"),
    }
}

// SECURITY: Constant-time PSS verification - uses error accumulation pattern
// to prevent timing side-channels per RFC 8017 requirements
pub fn verify_pss(msg: &[u8], sig: &[u8], key: &RsaPublicKey) -> bool {
    let hash = sha256(msg);

    let em_bits = key.bits - 1;
    let em_len = em_bits.div_ceil(8);
    let hash_len = 32;
    let salt_len = 32;

    // Accumulate validity flags - no early returns except for length (public info)
    let mut valid: u8 = 1;

    if sig.len() != key.bits / 8 {
        // Signature length is public information
        return false;
    }

    let signature = BigUint::from_bytes_be(sig);
    let em_big = match rsa_public_operation(&signature, key) {
        Ok(v) => v,
        Err(_) => {
            valid = 0;
            BigUint::from_u64(0)
        }
    };

    let mut em = em_big.to_bytes_be();
    while em.len() < em_len {
        em.insert(0, 0);
    }

    // Truncate or pad to exact length - track validity
    if em.len() != em_len {
        valid = 0;
        em.resize(em_len, 0);
    }

    // Check trailer byte (constant-time)
    valid &= ct_eq_u8(em[em_len - 1], 0xbc);

    let masked_db_len = em_len - hash_len - 1;
    let masked_db = &em[..masked_db_len];
    let h = &em[masked_db_len..em_len - 1];

    // Check top bits (constant-time)
    let top_bits = 8 * em_len - em_bits;
    if top_bits > 0 {
        let mask = 0xFFu8 >> (8 - top_bits);
        let top_ok = ct_eq_u8(masked_db[0] & mask, 0);
        valid &= top_ok;
    }

    // Always compute DB regardless of validity
    let db_mask = mgf1(h, masked_db_len);
    let mut db: Vec<u8> = masked_db.iter().zip(db_mask.iter()).map(|(a, b)| a ^ b).collect();

    if top_bits > 0 {
        db[0] &= 0xFFu8 >> top_bits;
    }

    // Constant-time padding verification
    let ps_len = em_len - hash_len - salt_len - 2;
    let mut padding_ok: u8 = 1;
    for &byte in &db[..ps_len] {
        padding_ok &= ct_eq_u8(byte, 0x00);
    }
    valid &= padding_ok;

    // Check separator byte
    valid &= ct_eq_u8(db[ps_len], 0x01);

    // Always compute hash regardless of validity
    let salt = &db[ps_len + 1..];

    let mut m_prime = vec![0u8; 8];
    m_prime.extend_from_slice(&hash);
    m_prime.extend_from_slice(salt);

    let h_computed = sha256(&m_prime);

    // Constant-time hash comparison
    let mut hash_match: u8 = 1;
    for i in 0..hash_len {
        hash_match &= ct_eq_u8(h[i], h_computed[i]);
    }
    valid &= hash_match;

    valid == 1
}

#[inline]
fn ct_eq_u8(a: u8, b: u8) -> u8 {
    let diff = a ^ b;
    let is_zero = (diff as u16 | (diff as u16).wrapping_neg()) >> 8;
    (1 ^ is_zero) as u8
}

/// MGF1 using SHA-384 as the underlying hash function.
fn mgf1_sha384(seed: &[u8], mask_len: usize) -> Vec<u8> {
    let mut mask = Vec::with_capacity(mask_len);
    let mut counter = 0u32;
    while mask.len() < mask_len {
        let mut input = seed.to_vec();
        input.extend_from_slice(&counter.to_be_bytes());
        let hash = sha384(&input);
        mask.extend_from_slice(&hash);
        counter += 1;
    }
    mask.truncate(mask_len);
    mask
}

// SECURITY: Constant-time PSS-SHA384 verification - uses error accumulation pattern
// to prevent timing side-channels per RFC 8017 requirements
pub fn verify_pss_sha384(msg: &[u8], sig: &[u8], key: &RsaPublicKey) -> bool {
    let hash = sha384(msg);

    let em_bits = key.bits - 1;
    let em_len = em_bits.div_ceil(8);
    let hash_len = 48;
    let salt_len = 48;

    let mut valid: u8 = 1;

    if sig.len() != key.bits / 8 {
        return false;
    }

    let signature = BigUint::from_bytes_be(sig);
    let em_big = match rsa_public_operation(&signature, key) {
        Ok(v) => v,
        Err(_) => {
            valid = 0;
            BigUint::from_u64(0)
        }
    };

    let mut em = em_big.to_bytes_be();
    while em.len() < em_len {
        em.insert(0, 0);
    }

    if em.len() != em_len {
        valid = 0;
        em.resize(em_len, 0);
    }

    valid &= ct_eq_u8(em[em_len - 1], 0xbc);

    let masked_db_len = em_len - hash_len - 1;
    let masked_db = &em[..masked_db_len];
    let h = &em[masked_db_len..em_len - 1];

    let top_bits = 8 * em_len - em_bits;
    if top_bits > 0 {
        let mask = 0xFFu8 >> (8 - top_bits);
        let top_ok = ct_eq_u8(masked_db[0] & mask, 0);
        valid &= top_ok;
    }

    let db_mask = mgf1_sha384(h, masked_db_len);
    let mut db: Vec<u8> = masked_db.iter().zip(db_mask.iter()).map(|(a, b)| a ^ b).collect();

    if top_bits > 0 {
        db[0] &= 0xFFu8 >> top_bits;
    }

    let ps_len = em_len - hash_len - salt_len - 2;
    let mut padding_ok: u8 = 1;
    for &byte in &db[..ps_len] {
        padding_ok &= ct_eq_u8(byte, 0x00);
    }
    valid &= padding_ok;

    valid &= ct_eq_u8(db[ps_len], 0x01);

    let salt = &db[ps_len + 1..];

    let mut m_prime = vec![0u8; 8];
    m_prime.extend_from_slice(&hash);
    m_prime.extend_from_slice(salt);

    let h_computed = sha384(&m_prime);

    let mut hash_match: u8 = 1;
    for i in 0..hash_len {
        hash_match &= ct_eq_u8(h[i], h_computed[i]);
    }
    valid &= hash_match;

    valid == 1
}
