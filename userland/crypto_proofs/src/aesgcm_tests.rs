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

use crate::crypto::symmetric::aes_gcm::{aes128_gcm_decrypt, aes128_gcm_encrypt};

extern crate alloc;
use alloc::vec::Vec;

// NIST AES-128-GCM known-answer vectors (McGrew & Viega GCM test cases 3 and 4,
// as adopted by the NIST GCM specification).

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

fn arr16(s: &str) -> [u8; 16] {
    let v = unhex(s);
    let mut a = [0u8; 16];
    a.copy_from_slice(&v);
    a
}

fn arr12(s: &str) -> [u8; 12] {
    let v = unhex(s);
    let mut a = [0u8; 12];
    a.copy_from_slice(&v);
    a
}

const KEY: &str = "feffe9928665731c6d6a8f9467308308";
const IV: &str = "cafebabefacedbaddecaf888";

#[test]
fn gcm_test_case_3_no_aad() {
    let pt = unhex(
        "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721\
         c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255",
    );
    let expect = unhex(
        "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e\
         21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091473f5985\
         4d5c2af327cd64a62cf35abd2ba6fab4",
    );
    let out = aes128_gcm_encrypt(&arr16(KEY), &arr12(IV), b"", &pt).unwrap();
    assert_eq!(out, expect, "ciphertext || tag");

    let back = aes128_gcm_decrypt(&arr16(KEY), &arr12(IV), b"", &out).unwrap();
    assert_eq!(back, pt, "round-trip plaintext");
}

#[test]
fn gcm_test_case_4_with_aad() {
    let aad = unhex("feedfacedeadbeeffeedfacedeadbeefabaddad2");
    let pt = unhex(
        "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721\
         c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39",
    );
    let expect = unhex(
        "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e\
         21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091\
         5bc94fbc3221a5db94fae95ae7121a47",
    );
    let out = aes128_gcm_encrypt(&arr16(KEY), &arr12(IV), &aad, &pt).unwrap();
    assert_eq!(out, expect, "ciphertext || tag with AAD");

    let back = aes128_gcm_decrypt(&arr16(KEY), &arr12(IV), &aad, &out).unwrap();
    assert_eq!(back, pt, "round-trip plaintext with AAD");
}

#[test]
fn gcm_decrypt_rejects_tampering() {
    let aad = unhex("feedfacedeadbeeffeedfacedeadbeefabaddad2");
    let pt = unhex("d9313225f88406e5a55909c5aff5269a");
    let out = aes128_gcm_encrypt(&arr16(KEY), &arr12(IV), &aad, &pt).unwrap();

    // Flipped tag byte.
    let mut bad = out.clone();
    let last = bad.len() - 1;
    bad[last] ^= 0x01;
    assert!(aes128_gcm_decrypt(&arr16(KEY), &arr12(IV), &aad, &bad).is_err());

    // Altered AAD.
    let mut bad_aad = aad.clone();
    bad_aad[0] ^= 0x01;
    assert!(aes128_gcm_decrypt(&arr16(KEY), &arr12(IV), &bad_aad, &out).is_err());
}
