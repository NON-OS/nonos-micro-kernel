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

// AES-128-GCM AEAD (NIST SP 800-38D) with a 96-bit nonce, the shape TLS 1.3
// records use. Tags are always 16 bytes.

use alloc::vec::Vec;

use super::aes128::{encrypt_block, expand, RoundKeys};
use super::ghash::ghash;

const TAG_LEN: usize = 16;

fn inc32(block: &mut [u8; 16]) {
    let ctr = u32::from_be_bytes([block[12], block[13], block[14], block[15]]).wrapping_add(1);
    block[12..].copy_from_slice(&ctr.to_be_bytes());
}

// XOR the CTR keystream, starting from inc32(J0), into `data` in place.
fn ctr_xor(keys: &RoundKeys, j0: &[u8; 16], data: &mut [u8]) {
    let mut ctr = *j0;
    for chunk in data.chunks_mut(16) {
        inc32(&mut ctr);
        let ks = encrypt_block(keys, &ctr);
        for (b, k) in chunk.iter_mut().zip(ks.iter()) {
            *b ^= *k;
        }
    }
}

fn tag(keys: &RoundKeys, h: &[u8; 16], j0: &[u8; 16], aad: &[u8], ct: &[u8]) -> [u8; 16] {
    let s = ghash(h, aad, ct);
    let ej0 = encrypt_block(keys, j0);
    let mut t = [0u8; 16];
    let mut i = 0;
    while i < 16 {
        t[i] = s[i] ^ ej0[i];
        i += 1;
    }
    t
}

fn setup(key: &[u8; 16], iv: &[u8; 12]) -> (RoundKeys, [u8; 16], [u8; 16]) {
    let keys = expand(key);
    let h = encrypt_block(&keys, &[0u8; 16]);
    let mut j0 = [0u8; 16];
    j0[..12].copy_from_slice(iv);
    j0[15] = 1;
    (keys, h, j0)
}

// Encrypt `plaintext`, returning ciphertext followed by the 16-byte tag.
pub fn seal(key: &[u8; 16], iv: &[u8; 12], aad: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let (keys, h, j0) = setup(key, iv);
    let mut out = Vec::with_capacity(plaintext.len() + TAG_LEN);
    out.extend_from_slice(plaintext);
    ctr_xor(&keys, &j0, &mut out);
    let t = tag(&keys, &h, &j0, aad, &out);
    out.extend_from_slice(&t);
    out
}

// Verify the tag over `ciphertext` (ct || tag) and, on success, return the
// decrypted plaintext. Fail closed on any length or authentication error.
pub fn open(key: &[u8; 16], iv: &[u8; 12], aad: &[u8], ciphertext: &[u8]) -> Option<Vec<u8>> {
    if ciphertext.len() < TAG_LEN {
        return None;
    }
    let (ct, recv_tag) = ciphertext.split_at(ciphertext.len() - TAG_LEN);
    let (keys, h, j0) = setup(key, iv);
    let want = tag(&keys, &h, &j0, aad, ct);
    let mut diff = 0u8;
    let mut i = 0;
    while i < TAG_LEN {
        diff |= want[i] ^ recv_tag[i];
        i += 1;
    }
    if diff != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(ct.len());
    out.extend_from_slice(ct);
    ctr_xor(&keys, &j0, &mut out);
    Some(out)
}
