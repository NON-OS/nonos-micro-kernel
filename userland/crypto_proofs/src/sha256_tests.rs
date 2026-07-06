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

use crate::hash::sha256::sha256;
use crate::hex::hex32;

// NIST FIPS 180-4 / standard SHA-256 known-answer vectors.

#[test]
fn sha256_empty() {
    assert_eq!(
        sha256(b""),
        hex32("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    );
}

#[test]
fn sha256_abc() {
    assert_eq!(
        sha256(b"abc"),
        hex32("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
    );
}

#[test]
fn sha256_448_bit_message() {
    assert_eq!(
        sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        hex32("248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1")
    );
}

#[test]
fn sha256_896_bit_message() {
    assert_eq!(
        sha256(
            b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn\
              hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
        ),
        hex32("cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1")
    );
}

#[test]
fn sha256_one_million_a() {
    // FIPS 180-4 long-message vector: 1,000,000 'a' bytes.
    let msg = alloc_a(1_000_000);
    assert_eq!(
        sha256(&msg),
        hex32("cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0")
    );
}

#[test]
fn sha256_is_deterministic() {
    // Same input always yields the same digest.
    let a = sha256(b"nonos capability microkernel");
    let b = sha256(b"nonos capability microkernel");
    assert_eq!(a, b);
    // A one-bit change avalanches (digests differ).
    assert_ne!(sha256(b"nonos"), sha256(b"nonoS"));
}

extern crate alloc;
fn alloc_a(n: usize) -> alloc::vec::Vec<u8> {
    alloc::vec![b'a'; n]
}
