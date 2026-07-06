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

use crate::hash::hmac::{hmac_sha256, hmac_verify};
use crate::hex::hex32;

extern crate alloc;
use alloc::vec::Vec;

// RFC 4231 HMAC-SHA-256 known-answer vectors. Each checks the kernel's real
// `hmac_sha256` against the standard's expected MAC (truncated to the SHA-256
// length where the RFC gives a longer digest for the SHA-2 family).

fn repeat(byte: u8, n: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(n);
    v.resize(n, byte);
    v
}

#[test]
fn rfc4231_case1() {
    let key = repeat(0x0b, 20);
    assert_eq!(
        hmac_sha256(&key, b"Hi There"),
        hex32("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7")
    );
}

#[test]
fn rfc4231_case2_short_key() {
    // Key shorter than the block, ASCII message.
    assert_eq!(
        hmac_sha256(b"Jefe", b"what do ya want for nothing?"),
        hex32("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843")
    );
}

#[test]
fn rfc4231_case3_full_block() {
    let key = repeat(0xaa, 20);
    let data = repeat(0xdd, 50);
    assert_eq!(
        hmac_sha256(&key, &data),
        hex32("773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe")
    );
}

#[test]
fn rfc4231_case4_numbered_key() {
    let key: Vec<u8> = (1u8..=25).collect();
    let data = repeat(0xcd, 50);
    assert_eq!(
        hmac_sha256(&key, &data),
        hex32("82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b")
    );
}

#[test]
fn rfc4231_case6_key_larger_than_block() {
    // 131-byte key forces the "hash the key first" path (key.len() > 64).
    let key = repeat(0xaa, 131);
    assert_eq!(
        hmac_sha256(&key, b"Test Using Larger Than Block-Size Key - Hash Key First"),
        hex32("60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54")
    );
}

#[test]
fn rfc4231_case7_large_key_and_data() {
    let key = repeat(0xaa, 131);
    let data = b"This is a test using a larger than block-size key and a larger \
                 than block-size data. The key needs to be hashed before being \
                 used by the HMAC algorithm.";
    assert_eq!(
        hmac_sha256(&key, data),
        hex32("9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2")
    );
}

#[test]
fn hmac_verify_accepts_valid_and_rejects_tampered() {
    let key = repeat(0x0b, 20);
    let mac = hmac_sha256(&key, b"Hi There");
    // Constant-time verify accepts the genuine MAC.
    assert!(hmac_verify(&key, b"Hi There", &mac));
    // A single flipped bit in the MAC is rejected.
    let mut bad = mac;
    bad[0] ^= 0x01;
    assert!(!hmac_verify(&key, b"Hi There", &bad));
    // A changed message is rejected under the same MAC.
    assert!(!hmac_verify(&key, b"Hi there", &mac));
    // A wrong key is rejected.
    let wrong = repeat(0x0c, 20);
    assert!(!hmac_verify(&wrong, b"Hi There", &mac));
}
