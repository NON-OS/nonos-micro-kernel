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

use crate::crypto::asymmetric::p384::{verify, PublicKey, Signature};
use crate::hash::sha384::sha384;

extern crate alloc;

// RFC 6979 Appendix A.2.6 ECDSA P-384 with SHA-384 known-answer vector.

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

const QX: &str = "EC3A4E415B4E19A4568618029F427FA5DA9A8BC4AE92E02E06AAE5286B300C64\
                  DEF8F0EA9055866064A254515480BC13";
const QY: &str = "8015D9B72D7D57244EA8EF9AC0C621896708A59367F9DFB9F54CA84B3F1C9DB1\
                  288B231C3AE0D4FE7344FD2533264720";

fn public_key() -> PublicKey {
    let x: [u8; 48] = bytes(QX);
    let y: [u8; 48] = bytes(QY);
    let mut pk: PublicKey = [0u8; 97];
    pk[0] = 0x04;
    pk[1..49].copy_from_slice(&x);
    pk[49..97].copy_from_slice(&y);
    pk
}

fn signature() -> Signature {
    let r: [u8; 48] = bytes(
        "94EDBB92A5ECB8AAD4736E56C691916B3F88140666CE9FA73D64C4EA95AD133C\
         81A648152E44ACF96E36DD1E80FABE46",
    );
    let s: [u8; 48] = bytes(
        "99EF4AEB15F178CEA1FE40DB2603138F130E740A19624526203B6351D0A3A94F\
         A329C145786E679E7B82C71A38628AC8",
    );
    let mut sig: Signature = [0u8; 96];
    sig[..48].copy_from_slice(&r);
    sig[48..].copy_from_slice(&s);
    sig
}

#[test]
fn rfc6979_p384_sha384_sample() {
    assert!(verify(&public_key(), &sha384(b"sample"), &signature()));
}

#[test]
fn p384_verify_rejects_tampering() {
    assert!(verify(&public_key(), &sha384(b"sample"), &signature()));
    // Wrong message.
    assert!(!verify(&public_key(), &sha384(b"test"), &signature()));
    // Flipped byte in r and in s.
    let mut bad = signature();
    bad[0] ^= 0x01;
    assert!(!verify(&public_key(), &sha384(b"sample"), &bad));
    let mut bad_s = signature();
    bad_s[95] ^= 0x01;
    assert!(!verify(&public_key(), &sha384(b"sample"), &bad_s));
}
