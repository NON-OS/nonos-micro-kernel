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

use crate::crypto::asymmetric::secp256k1::{public_key_from_secret, sign, verify, Signature};
use crate::hash::sha256::sha256;

// secp256k1 is the chain signature curve. These proofs anchor the scalar
// multiplication on the standard generator point and exercise the real
// RFC 6979 deterministic sign/verify over the curve.

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("bad hex digit"),
    }
}

fn hex32(s: &str) -> [u8; 32] {
    let h = s.as_bytes();
    assert_eq!(h.len(), 64, "hex length");
    let mut out = [0u8; 32];
    for (i, o) in out.iter_mut().enumerate() {
        *o = (nibble(h[2 * i]) << 4) | nibble(h[2 * i + 1]);
    }
    out
}

// SEC 2 standard generator G for secp256k1.
const GX: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
const GY: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";

fn secret_one() -> [u8; 32] {
    let mut sk = [0u8; 32];
    sk[31] = 1;
    sk
}

#[test]
fn scalar_mult_of_one_is_the_standard_generator() {
    // Public key for the scalar 1 must be exactly the SEC 2 generator point.
    let pk = public_key_from_secret(&secret_one()).expect("valid secret");
    assert_eq!(pk[0], 0x04, "uncompressed prefix");
    assert_eq!(&pk[1..33], &hex32(GX), "generator x");
    assert_eq!(&pk[33..65], &hex32(GY), "generator y");
}

#[test]
fn deterministic_sign_then_verify_roundtrips() {
    let sk = hex32("C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721");
    let pk = public_key_from_secret(&sk).expect("valid secret");
    let digest = sha256(b"nonos chain settlement");

    let rec = sign(&sk, &digest).expect("sign");
    let mut sig: Signature = [0u8; 64];
    sig[..32].copy_from_slice(&rec.r);
    sig[32..].copy_from_slice(&rec.s);

    assert!(verify(&pk, &digest, &sig), "genuine signature verifies");

    // Determinism: the RFC 6979 nonce yields the same signature every time.
    let rec2 = sign(&sk, &digest).expect("sign");
    assert_eq!(rec.r, rec2.r);
    assert_eq!(rec.s, rec2.s);
}

#[test]
fn verify_rejects_tampering() {
    let sk = hex32("C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721");
    let pk = public_key_from_secret(&sk).expect("valid secret");
    let digest = sha256(b"nonos chain settlement");
    let rec = sign(&sk, &digest).expect("sign");
    let mut sig: Signature = [0u8; 64];
    sig[..32].copy_from_slice(&rec.r);
    sig[32..].copy_from_slice(&rec.s);

    assert!(verify(&pk, &digest, &sig));
    // Different message digest is rejected.
    assert!(!verify(&pk, &sha256(b"different message"), &sig));
    // Flipped byte in r and in s is rejected.
    let mut bad = sig;
    bad[0] ^= 0x01;
    assert!(!verify(&pk, &digest, &bad));
    let mut bad_s = sig;
    bad_s[63] ^= 0x01;
    assert!(!verify(&pk, &digest, &bad_s));
}
