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

use crate::hash::sha512::sha512;

// NIST FIPS 180-4 SHA-512 known-answer vectors.

fn hex64(s: &str) -> [u8; 64] {
    let b = s.as_bytes();
    assert_eq!(b.len(), 128, "expected a 128-char hex digest");
    let mut out = [0u8; 64];
    for (i, byte) in out.iter_mut().enumerate() {
        out_nibbles(byte, b[2 * i], b[2 * i + 1]);
    }
    out
}

fn out_nibbles(byte: &mut u8, hi: u8, lo: u8) {
    *byte = (nibble(hi) << 4) | nibble(lo);
}

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => panic!("bad hex digit"),
    }
}

#[test]
fn sha512_empty() {
    assert_eq!(
        sha512(b""),
        hex64(
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
             47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
        )
    );
}

#[test]
fn sha512_abc() {
    assert_eq!(
        sha512(b"abc"),
        hex64(
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        )
    );
}

#[test]
fn sha512_896_bit_message() {
    assert_eq!(
        sha512(
            b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn\
              hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
        ),
        hex64(
            "8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018\
             501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909"
        )
    );
}
