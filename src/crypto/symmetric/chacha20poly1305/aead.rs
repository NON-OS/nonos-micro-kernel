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

extern crate alloc;

use super::chacha20::{chacha20_block, chacha20_xor, secure_zero_bytes};
use super::poly1305::Poly1305;
use crate::crypto::constant_time::ct_eq;
use alloc::vec::Vec;

pub(crate) const TAG_SIZE: usize = 16;

pub fn aead_encrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, &'static str> {
    let mut block0 = [0u8; 64];
    chacha20_block(key, nonce, 0, &mut block0);
    let mut otk = [0u8; 32];
    otk.copy_from_slice(&block0[..32]);

    let mut ciphertext = plaintext.to_vec();
    chacha20_xor(key, nonce, 1, &mut ciphertext);

    let tag = compute_tag(&otk, aad, &ciphertext);

    secure_zero_bytes(&mut otk);
    secure_zero_bytes(&mut block0);

    let mut result = ciphertext;
    result.extend_from_slice(&tag);
    Ok(result)
}

// SECURITY: Constant-time decrypt - always performs decryption to prevent timing oracle
pub fn aead_decrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    ciphertext_and_tag: &[u8],
) -> Result<Vec<u8>, &'static str> {
    if ciphertext_and_tag.len() < TAG_SIZE {
        return Err("ciphertext too short");
    }

    let ct_len = ciphertext_and_tag.len() - TAG_SIZE;
    let (ciphertext, tag) = ciphertext_and_tag.split_at(ct_len);

    let mut block0 = [0u8; 64];
    chacha20_block(key, nonce, 0, &mut block0);
    let mut otk = [0u8; 32];
    otk.copy_from_slice(&block0[..32]);

    let expected_tag = compute_tag(&otk, aad, ciphertext);
    let tag_ok = ct_eq(&expected_tag, tag);

    secure_zero_bytes(&mut otk);
    secure_zero_bytes(&mut block0);

    // SECURITY: Always decrypt regardless of tag validity to prevent timing oracle
    let mut plaintext = ciphertext.to_vec();
    chacha20_xor(key, nonce, 1, &mut plaintext);

    if !tag_ok {
        // SECURITY: Zero plaintext to prevent leaking decrypted data on auth failure
        secure_zero_bytes(&mut plaintext);
        return Err("tag mismatch");
    }

    Ok(plaintext)
}

pub(crate) fn compute_tag(otk: &[u8; 32], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
    let mut poly = Poly1305::new(otk);

    poly.update(aad);

    let aad_padding = (16 - (aad.len() % 16)) % 16;
    if aad_padding > 0 {
        let zeros = [0u8; 16];
        poly.update(&zeros[..aad_padding]);
    }

    poly.update(ciphertext);

    let ct_padding = (16 - (ciphertext.len() % 16)) % 16;
    if ct_padding > 0 {
        let zeros = [0u8; 16];
        poly.update(&zeros[..ct_padding]);
    }

    let mut lengths = [0u8; 16];
    lengths[0..8].copy_from_slice(&(aad.len() as u64).to_le_bytes());
    lengths[8..16].copy_from_slice(&(ciphertext.len() as u64).to_le_bytes());
    poly.update(&lengths);

    poly.finalize()
}

pub fn aead_encrypt_in_place(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    buffer: &mut [u8],
    plaintext_len: usize,
) -> Result<usize, &'static str> {
    if buffer.len() < plaintext_len + TAG_SIZE {
        return Err("buffer too small");
    }

    let mut block0 = [0u8; 64];
    chacha20_block(key, nonce, 0, &mut block0);
    let mut otk = [0u8; 32];
    otk.copy_from_slice(&block0[..32]);

    chacha20_xor(key, nonce, 1, &mut buffer[..plaintext_len]);

    let tag = compute_tag(&otk, aad, &buffer[..plaintext_len]);

    buffer[plaintext_len..plaintext_len + TAG_SIZE].copy_from_slice(&tag);

    secure_zero_bytes(&mut otk);
    secure_zero_bytes(&mut block0);

    Ok(plaintext_len + TAG_SIZE)
}

// SECURITY: Constant-time decrypt_in_place - always performs decryption to prevent timing oracle
pub fn aead_decrypt_in_place(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    buffer: &mut [u8],
    ciphertext_and_tag_len: usize,
) -> Result<usize, &'static str> {
    if ciphertext_and_tag_len < TAG_SIZE || buffer.len() < ciphertext_and_tag_len {
        return Err("invalid length");
    }

    let ct_len = ciphertext_and_tag_len - TAG_SIZE;
    let tag = &buffer[ct_len..ciphertext_and_tag_len].to_vec();

    let mut block0 = [0u8; 64];
    chacha20_block(key, nonce, 0, &mut block0);
    let mut otk = [0u8; 32];
    otk.copy_from_slice(&block0[..32]);

    let expected_tag = compute_tag(&otk, aad, &buffer[..ct_len]);
    let tag_ok = ct_eq(&expected_tag, tag);

    secure_zero_bytes(&mut otk);
    secure_zero_bytes(&mut block0);

    // SECURITY: Always decrypt regardless of tag validity to prevent timing oracle
    chacha20_xor(key, nonce, 1, &mut buffer[..ct_len]);

    if !tag_ok {
        // SECURITY: Zero buffer to prevent leaking decrypted data on auth failure
        secure_zero_bytes(&mut buffer[..ct_len]);
        return Err("tag mismatch");
    }

    Ok(ct_len)
}
