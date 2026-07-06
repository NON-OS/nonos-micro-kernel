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

use crate::crypto::asymmetric::ed25519::{sign, verify, KeyPair, Signature};

extern crate alloc;
use alloc::vec::Vec;

// Ed25519 is the trust root: the bootloader and capsule loader accept only
// signatures this code verifies. These proofs run the real sign/verify against
// RFC 8032 Section 7.1 known-answer vectors, plus adversarial edge cases.

fn unhex(s: &str) -> Vec<u8> {
    let digits: Vec<u8> = s.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    assert!(digits.len().is_multiple_of(2), "odd hex length");
    digits.chunks_exact(2).map(|pair| (nibble(pair[0]) << 4) | nibble(pair[1])).collect()
}

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("bad hex digit"),
    }
}

fn arr32(s: &str) -> [u8; 32] {
    let v = unhex(s);
    assert_eq!(v.len(), 32, "expected 32 bytes");
    let mut a = [0u8; 32];
    a.copy_from_slice(&v);
    a
}

fn arr64(s: &str) -> [u8; 64] {
    let v = unhex(s);
    assert_eq!(v.len(), 64, "expected 64 bytes");
    let mut a = [0u8; 64];
    a.copy_from_slice(&v);
    a
}

// (seed, public, message, signature) -> exercise the full RFC 8032 contract:
// public-key derivation, deterministic signing, and verification all match.
fn check_vector(seed: &str, public: &str, msg: &str, sig: &str) {
    let seed = arr32(seed);
    let public = arr32(public);
    let msg = unhex(msg);
    let sig = Signature::from_bytes(&arr64(sig));

    // Public key is derived from the seed exactly as the standard specifies.
    let kp = KeyPair::from_seed(seed);
    assert_eq!(kp.public, public, "public key derivation");

    // Ed25519 signing is deterministic: the produced signature is the vector's.
    let produced = sign(&kp, &msg);
    assert_eq!(produced.to_bytes(), sig.to_bytes(), "deterministic signature");

    // And the standard signature verifies under the standard public key.
    assert!(verify(&public, &msg, &sig), "verify");
}

#[test]
fn rfc8032_test1_empty_message() {
    check_vector(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
        "",
        "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
    );
}

#[test]
fn rfc8032_test2_one_byte() {
    check_vector(
        "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
        "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
        "72",
        "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00",
    );
}

#[test]
fn rfc8032_test3_two_bytes() {
    check_vector(
        "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
        "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
        "af82",
        "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a",
    );
}

#[test]
fn rfc8032_test_sha_abc() {
    check_vector(
        "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42",
        "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
        "ddaf35a193617abacc417349ae204131\
         12e6fa4e89a97ea20a9eeee64b55d39a\
         2192992a274fc1a836ba3c23a3feebbd\
         454d4423643ce80e2a9ac94fa54ca49f",
        "dc2a4459e7369633a52b1bf277839a00201009a3efbf3ecb69bea2186c26b58909351fc9ac90b3ecfdfbc7c66431e0303dca179c138ac17ad9bef1177331a704",
    );
}

#[test]
fn verify_rejects_tampering() {
    let seed = arr32("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
    let kp = KeyPair::from_seed(seed);
    let msg = b"nonos capsule manifest v1";
    let sig = sign(&kp, msg);

    // Genuine signature verifies.
    assert!(verify(&kp.public, msg, &sig));
    // Any single-bit change to the message is rejected.
    assert!(!verify(&kp.public, b"nonos capsule manifest v2", &sig));
    // A flipped bit in S is rejected.
    let mut bad = sig.clone();
    bad.S[0] ^= 0x01;
    assert!(!verify(&kp.public, msg, &bad));
    // A flipped bit in R is rejected.
    let mut bad_r = sig.clone();
    bad_r.R[0] ^= 0x01;
    assert!(!verify(&kp.public, msg, &bad_r));
    // A different public key does not verify the signature.
    let other = KeyPair::from_seed([9u8; 32]);
    assert!(!verify(&other.public, msg, &sig));
}
