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

use crate::hash::hkdf::{hkdf_expand, hkdf_extract};
use crate::hex::hex32;

extern crate alloc;
use alloc::vec::Vec;

// RFC 5869 HKDF-SHA256 known-answer vectors (Appendix A).

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

#[test]
fn rfc5869_case1_basic() {
    let ikm = unhex("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");
    let salt = unhex("000102030405060708090a0b0c");
    let info = unhex("f0f1f2f3f4f5f6f7f8f9");

    let prk = hkdf_extract(Some(&salt), &ikm);
    assert_eq!(
        prk,
        hex32("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5"),
        "PRK (extract)"
    );

    let mut okm = [0u8; 42];
    hkdf_expand(&prk, &info, &mut okm).unwrap();
    assert_eq!(
        &okm[..],
        &unhex(
            "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf\
             34007208d5b887185865"
        )[..],
        "OKM (expand)"
    );
}

#[test]
fn rfc5869_case3_zero_salt_and_info() {
    let ikm = unhex("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");

    // No salt provided: HKDF sets it to a string of HashLen zeros.
    let prk = hkdf_extract(None, &ikm);
    assert_eq!(
        prk,
        hex32("19ef24a32c717b167f33a91d6f648bdf96596776afdb6377ac434c1c293ccb04"),
        "PRK (extract, no salt)"
    );

    let mut okm = [0u8; 42];
    hkdf_expand(&prk, b"", &mut okm).unwrap();
    assert_eq!(
        &okm[..],
        &unhex(
            "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d\
             9d201395faa4b61a96c8"
        )[..],
        "OKM (expand, no info)"
    );
}

#[test]
fn hkdf_expand_rejects_oversized_output() {
    let prk = hkdf_extract(Some(b"salt"), b"ikm");
    let mut too_big = [0u8; 255 * 32 + 1];
    assert!(hkdf_expand(&prk, b"", &mut too_big).is_err());
}
