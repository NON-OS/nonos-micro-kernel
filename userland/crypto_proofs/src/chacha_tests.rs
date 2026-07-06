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

use crate::crypto::symmetric::chacha20poly1305::{aead_decrypt, aead_encrypt};

extern crate alloc;
use alloc::vec::Vec;

// RFC 8439 Section 2.8.2 ChaCha20-Poly1305 AEAD known-answer vector.

fn unhex(s: &str) -> Vec<u8> {
    let b: Vec<u8> = s.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
    assert!(b.len().is_multiple_of(2), "odd hex length");
    b.chunks_exact(2).map(|p| (nibble(p[0]) << 4) | nibble(p[1])).collect()
}

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => panic!("bad hex digit"),
    }
}

fn arr32(s: &str) -> [u8; 32] {
    let v = unhex(s);
    let mut a = [0u8; 32];
    a.copy_from_slice(&v);
    a
}

fn arr12(s: &str) -> [u8; 12] {
    let v = unhex(s);
    let mut a = [0u8; 12];
    a.copy_from_slice(&v);
    a
}

const KEY: &str = "808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f";
const NONCE: &str = "070000004041424344454647";
const AAD: &str = "50515253c0c1c2c3c4c5c6c7";
const PLAINTEXT: &[u8] = b"Ladies and Gentlemen of the class of '99: If I could offer you \
                           only one tip for the future, sunscreen would be it.";
// Ciphertext followed by the 16-byte Poly1305 tag, from the RFC.
const CT_AND_TAG: &str = "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6\
                          3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36\
                          92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc\
                          3ff4def08e4b7a9de576d26586cec64b6116\
                          1ae10b594f09e26a7e902ecbd0600691";

#[test]
fn rfc8439_aead_encrypt_matches_vector() {
    let out = aead_encrypt(&arr32(KEY), &arr12(NONCE), &unhex(AAD), PLAINTEXT).unwrap();
    assert_eq!(out, unhex(CT_AND_TAG), "ciphertext || tag");
}

#[test]
fn rfc8439_aead_decrypt_recovers_plaintext() {
    let pt = aead_decrypt(&arr32(KEY), &arr12(NONCE), &unhex(AAD), &unhex(CT_AND_TAG)).unwrap();
    assert_eq!(pt, PLAINTEXT, "decrypted plaintext");
}

#[test]
fn aead_decrypt_rejects_tampering() {
    let key = arr32(KEY);
    let nonce = arr12(NONCE);
    let aad = unhex(AAD);

    // A flipped tag byte is rejected.
    let mut bad_tag = unhex(CT_AND_TAG);
    let last = bad_tag.len() - 1;
    bad_tag[last] ^= 0x01;
    assert!(aead_decrypt(&key, &nonce, &aad, &bad_tag).is_err());

    // A flipped ciphertext byte is rejected.
    let mut bad_ct = unhex(CT_AND_TAG);
    bad_ct[0] ^= 0x01;
    assert!(aead_decrypt(&key, &nonce, &aad, &bad_ct).is_err());

    // Altered associated data is rejected.
    let mut bad_aad = aad.clone();
    bad_aad[0] ^= 0x01;
    assert!(aead_decrypt(&key, &nonce, &bad_aad, &unhex(CT_AND_TAG)).is_err());
}
