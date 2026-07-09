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

use crate::crypto::asymmetric::p256::{verify, PublicKey, Signature};
use crate::hash::sha256::sha256;

// RFC 6979 Appendix A.2.5 ECDSA P-256 with SHA-256 known-answer vectors. The
// signatures are verified against the real P-256 verify over the message's
// SHA-256 digest.

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("bad hex digit"),
    }
}

fn bytes<const N: usize>(s: &str) -> [u8; N] {
    let h: alloc::vec::Vec<u8> = s.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
    assert_eq!(h.len(), 2 * N, "hex length");
    let mut out = [0u8; N];
    for (i, o) in out.iter_mut().enumerate() {
        *o = (nibble(h[2 * i]) << 4) | nibble(h[2 * i + 1]);
    }
    out
}

extern crate alloc;

// Public key U (uncompressed SEC1: 0x04 || X || Y) shared by both vectors.
const QX: &str = "60FED4BA255A9D31C961EB74C6356D68C049B8923B61FA6CE669622E60F29FB6";
const QY: &str = "7903FE1008B8BC99A41AE9E95628BC64F2F1B20C2D7E9F5177A3C294D4462299";

fn public_key() -> PublicKey {
    let x: [u8; 32] = bytes(QX);
    let y: [u8; 32] = bytes(QY);
    let mut pk: PublicKey = [0u8; 65];
    pk[0] = 0x04;
    pk[1..33].copy_from_slice(&x);
    pk[33..65].copy_from_slice(&y);
    pk
}

fn signature(r: &str, s: &str) -> Signature {
    let rb: [u8; 32] = bytes(r);
    let sb: [u8; 32] = bytes(s);
    let mut sig: Signature = [0u8; 64];
    sig[..32].copy_from_slice(&rb);
    sig[32..].copy_from_slice(&sb);
    sig
}

#[test]
fn rfc6979_p256_sha256_sample() {
    let sig = signature(
        "EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716",
        "F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8",
    );
    assert!(verify(&public_key(), &sha256(b"sample"), &sig));
}

#[test]
fn rfc6979_p256_sha256_test() {
    let sig = signature(
        "F1ABB023518351CD71D881567B1EA663ED3EFCF6C5132B354F28D3B0B7D38367",
        "019F4113742A2B14BD25926B49C649155F267E60D3814B4C0CC84250E46F0083",
    );
    assert!(verify(&public_key(), &sha256(b"test"), &sig));
}

#[test]
fn p256_verify_rejects_tampering() {
    let sig = signature(
        "EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716",
        "F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8",
    );
    // Genuine signature verifies.
    assert!(verify(&public_key(), &sha256(b"sample"), &sig));
    // The "sample" signature does not verify over the "test" digest.
    assert!(!verify(&public_key(), &sha256(b"test"), &sig));
    // A flipped byte in r is rejected.
    let mut bad = sig;
    bad[0] ^= 0x01;
    assert!(!verify(&public_key(), &sha256(b"sample"), &bad));
    // A flipped byte in s is rejected.
    let mut bad_s = sig;
    bad_s[63] ^= 0x01;
    assert!(!verify(&public_key(), &sha256(b"sample"), &bad_s));
}
