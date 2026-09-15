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
use crate::crypto::util::bigint::BigUint;
use crate::crypto::{CryptoError, CryptoResult};
use alloc::vec::Vec;

pub fn oaep_encrypt(public_key: &RsaPublicKey, plaintext: &[u8]) -> CryptoResult<Vec<u8>> {
    let k = public_key.bits / 8;
    let hash_len = 32;

    if plaintext.len() > k - 2 * hash_len - 2 {
        return Err(CryptoError::InvalidLength);
    }

    let seed = get_entropy(hash_len);

    let mut db = Vec::with_capacity(k - hash_len - 1);
    db.extend_from_slice(&sha256(b""));

    let ps_len = k - plaintext.len() - 2 * hash_len - 2;
    db.resize(db.len() + ps_len, 0);
    db.push(0x01);
    db.extend_from_slice(plaintext);

    let db_mask = mgf1(&seed, db.len());
    let masked_db: Vec<u8> = db.iter().zip(db_mask.iter()).map(|(a, b)| a ^ b).collect();

    let seed_mask = mgf1(&masked_db, hash_len);
    let masked_seed: Vec<u8> = seed.iter().zip(seed_mask.iter()).map(|(a, b)| a ^ b).collect();

    let mut em = Vec::with_capacity(k);
    em.push(0x00);
    em.extend_from_slice(&masked_seed);
    em.extend_from_slice(&masked_db);

    let message = BigUint::from_bytes_be(&em);
    let ciphertext = rsa_public_operation(&message, public_key)?;

    let mut result = ciphertext.to_bytes_be();

    while result.len() < k {
        result.insert(0, 0);
    }

    Ok(result)
}

// SAFETY: This implementation uses constant-time validation to prevent
// Manger's attack, which exploits timing differences in padding validation.
// All error conditions are accumulated and checked at the end.
pub fn oaep_decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> CryptoResult<Vec<u8>> {
    let k = private_key.bits / 8;
    let hash_len = 32;

    if ciphertext.len() != k {
        return Err(CryptoError::InvalidLength);
    }

    let c = BigUint::from_bytes_be(ciphertext);
    let em_big = rsa_private_operation(&c, private_key)?;
    let mut em = em_big.to_bytes_be();

    while em.len() < k {
        em.insert(0, 0);
    }

    let mut error_mask: u8 = 0;

    error_mask |= em[0];

    let masked_seed = &em[1..hash_len + 1];
    let masked_db = &em[hash_len + 1..];

    let seed_mask = mgf1(masked_db, hash_len);
    let seed: Vec<u8> = masked_seed.iter().zip(seed_mask.iter()).map(|(a, b)| a ^ b).collect();

    let db_mask = mgf1(&seed, masked_db.len());
    let db: Vec<u8> = masked_db.iter().zip(db_mask.iter()).map(|(a, b)| a ^ b).collect();

    let lhash = sha256(b"");
    for i in 0..hash_len {
        error_mask |= db[i] ^ lhash[i];
    }

    let mut sep_idx: usize = 0;
    let mut found_sep: u8 = 0;
    let mut invalid_padding: u8 = 0;

    for (i, &byte) in db.iter().enumerate().skip(hash_len) {
        let is_zero = ct_is_zero(byte);
        let is_one = ct_is_eq(byte, 0x01);

        let should_update = (!found_sep) & is_one;
        sep_idx = ct_select_usize(should_update, i, sep_idx);

        found_sep |= is_one & (!found_sep.wrapping_sub(1));

        let in_padding = !found_sep.wrapping_sub(1);
        invalid_padding |= in_padding & (!is_zero) & (!is_one);
    }

    error_mask |= invalid_padding;
    error_mask |= !found_sep.wrapping_sub(1);

    if error_mask != 0 {
        return Err(CryptoError::InvalidLength);
    }

    Ok(db[sep_idx + 1..].to_vec())
}

#[inline]
fn ct_is_zero(x: u8) -> u8 {
    let x_minus_1 = x.wrapping_sub(1);
    !x & (x_minus_1 >> 7).wrapping_neg()
}

#[inline]
fn ct_is_eq(a: u8, b: u8) -> u8 {
    ct_is_zero(a ^ b)
}

#[inline]
fn ct_select_usize(mask: u8, a: usize, b: usize) -> usize {
    let m = (mask as usize).wrapping_neg();
    (a & m) | (b & !m)
}

pub fn encrypt(data: &[u8], key: &RsaPublicKey) -> Result<Vec<u8>, &'static str> {
    if data.len() > (key.n.bits() / 8) - 11 {
        return Err("Data too large for RSA key");
    }

    let padded = pkcs1_v15_encrypt_pad(data, key.n.bits() / 8)?;
    let padded_int = BigUint::from_bytes_be(&padded);

    let encrypted = padded_int.mod_pow(&key.e, &key.n).ok_or("RSA encryption failed")?;
    Ok(encrypted.to_bytes_be())
}

pub fn decrypt(data: &[u8], key: &RsaPrivateKey) -> Result<Vec<u8>, &'static str> {
    oaep_decrypt(key, data).map_err(|_| "RSA decryption failed")
}

fn pkcs1_v15_encrypt_pad(data: &[u8], key_size: usize) -> Result<Vec<u8>, &'static str> {
    if data.len() + 11 > key_size {
        return Err("Data too long for PKCS#1 padding");
    }

    let mut padded = alloc::vec![0u8; key_size];
    padded[0] = 0x00;
    padded[1] = 0x02;

    let padding_len = key_size - data.len() - 3;
    for slot in padded.iter_mut().take(2 + padding_len).skip(2) {
        loop {
            let random = get_entropy(1);
            if random[0] != 0 {
                *slot = random[0];
                break;
            }
        }
    }

    padded[2 + padding_len] = 0x00;
    padded[(3 + padding_len)..].copy_from_slice(data);

    Ok(padded)
}
