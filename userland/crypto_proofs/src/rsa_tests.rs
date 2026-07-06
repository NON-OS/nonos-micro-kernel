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

use crate::crypto::asymmetric::rsa::{create_public_key, verify_pkcs1v15};

extern crate alloc;
use alloc::vec::Vec;

// Cross-implementation known-answer test: a 2048-bit RSASSA-PKCS1-v1.5 (SHA-256)
// signature produced by OpenSSL 3.6.2 must verify under the kernel's own RSA
// verify. This proves interop against a reference implementation with a real
// key, no self-signing.

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

// n = modulus, e = 65537, of the OpenSSL-generated 2048-bit key.
const N: &str = "c8b40b65641d759b62534209fd3e590dccf0290b778aaab9e658a3ba6c93d544\
                 b254d1e678a901993e10b82506370795af5628a5d59a4d488ed63c7985cebd29\
                 933eddbc651706fb26e62df672f39ed2ecbfd1727538c7d592b8368ff551a903\
                 9e0543d9525781e64f419e89c216600d0a8cae1bf982355f50be9cfbcda200ace\
                 1abe447c24322ac1cc1e24e029abf8747c58625d684df031b5744edcfbc6a6db6\
                 1a89f0ca9acc0a01b184995482b2d94fe79fc069d3941fcf187048f3dee3215bb\
                 cf29a7e22e476526001789df93c39abe36e080786a8d0c9af8751d3688547450\
                 4a6397e232da7818e1a842f550e6b984acd79c74ca1ad1473821b01e7ea29";
const E: &str = "010001";

// RSASSA-PKCS1-v1.5 signature over "nonos rsa attestation" with SHA-256.
const SIG: &str = "1cd786856ef69216eebd164bf23ddb426ebc7e98afecdc23cc33f09a06b7f2d0\
                   196b159138fe21d68073f6fc0f70f70edc9d667ee135fa7a5e4bb3355558cc38\
                   c589340ec4a9a4f83d650edff6cd21656b11822ba3d83d3a1179af905af34f07\
                   bbd48554ab0190303c7b8c0ba3f1a2dd2950f985a00c27d80d89a4cb0df80701\
                   d9e6fbdf3d07afcd1e34c87b5f0ee486debdc71148256fddafaff2f98d1ee8c9\
                   cf7d1a50a45e95b028f5ac49c5c4c013e0a9c59dab7fee7b9661581044dc74f21\
                   023cecf9b00f03fd6c39752f8631432e7260aef6ced52337c04386565c8b8ff22\
                   b98021f36842008c2ba555b37c54e45aff2ca4205e6fa406d9c4a4197c6656";

const MSG: &[u8] = b"nonos rsa attestation";

#[test]
fn openssl_pkcs1v15_sha256_signature_verifies() {
    let key = create_public_key(unhex(N), unhex(E));
    assert!(verify_pkcs1v15(&key, MSG, &unhex(SIG)));
}

#[test]
fn verify_rejects_tampering() {
    let key = create_public_key(unhex(N), unhex(E));

    // A different message does not verify under the same signature.
    assert!(!verify_pkcs1v15(&key, b"nonos rsa attestatioN", &unhex(SIG)));

    // A flipped byte in the signature is rejected.
    let mut bad = unhex(SIG);
    bad[0] ^= 0x01;
    assert!(!verify_pkcs1v15(&key, MSG, &bad));

    // A flipped byte in the modulus (wrong key) is rejected.
    let mut bad_n = unhex(N);
    let last = bad_n.len() - 1;
    bad_n[last] ^= 0x01;
    let bad_key = create_public_key(bad_n, unhex(E));
    assert!(!verify_pkcs1v15(&bad_key, MSG, &unhex(SIG)));
}
