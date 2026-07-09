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

use crate::hash::blake3::{blake3_derive_key, blake3_hash, blake3_keyed_hash};
use crate::hex::hex32;

extern crate alloc;
use alloc::vec::Vec;

// Official BLAKE3 team test vectors (test_vectors.json). The test input of
// length n is the byte sequence i mod 251.

const KEY: &[u8; 32] = b"whats the Elvish word for friend";
const CONTEXT: &str = "BLAKE3 2019-12-27 16:29:52 test vectors context";

fn input(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i % 251) as u8).collect()
}

#[test]
fn blake3_hash_vectors() {
    assert_eq!(
        blake3_hash(&input(0)),
        hex32("af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262")
    );
    assert_eq!(
        blake3_hash(&input(1)),
        hex32("2d3adedff11b61f14c886e35afa036736dcd87a74d27b5c1510225d0f592e213")
    );
    assert_eq!(
        blake3_hash(&input(1024)),
        hex32("42214739f095a406f3fc83deb889744ac00df831c10daa55189b5d121c855af7")
    );
    assert_eq!(
        blake3_hash(&input(2048)),
        hex32("e776b6028c7cd22a4d0ba182a8bf62205d2ef576467e838ed6f2529b85fba24a")
    );
}

#[test]
fn blake3_keyed_hash_vectors() {
    assert_eq!(
        blake3_keyed_hash(KEY, &input(0)),
        hex32("92b2b75604ed3c761f9d6f62392c8a9227ad0ea3f09573e783f1498a4ed60d26")
    );
    assert_eq!(
        blake3_keyed_hash(KEY, &input(1)),
        hex32("6d7878dfff2f485635d39013278ae14f1454b8c0a3a2d34bc1ab38228a80c95b")
    );
}

#[test]
fn blake3_derive_key_vectors() {
    let mut out = [0u8; 32];

    blake3_derive_key(CONTEXT, &input(0), &mut out);
    assert_eq!(out, hex32("2cc39783c223154fea8dfb7c1b1660f2ac2dcbd1c1de8277b0b0dd39b7e50d7d"));

    blake3_derive_key(CONTEXT, &input(1), &mut out);
    assert_eq!(out, hex32("b3e2e340a117a499c6cf2398a19ee0d29cca2bb7404c73063382693bf66cb06c"));
}

#[test]
fn blake3_is_deterministic_and_avalanches() {
    assert_eq!(blake3_hash(b"nonos"), blake3_hash(b"nonos"));
    assert_ne!(blake3_hash(b"nonos"), blake3_hash(b"nonoS"));
    // Keyed hash under a different key differs from the plain hash.
    assert_ne!(blake3_keyed_hash(KEY, b"nonos"), blake3_hash(b"nonos"));
}
