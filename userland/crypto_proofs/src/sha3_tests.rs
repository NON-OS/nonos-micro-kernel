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

use crate::hash::sha3::{sha3_256, sha3_512};
use crate::hex::hex32;

// NIST FIPS 202 SHA-3 known-answer vectors.

fn hex64(s: &str) -> [u8; 64] {
    let b = s.as_bytes();
    assert_eq!(b.len(), 128, "expected a 128-char hex digest");
    let mut out = [0u8; 64];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = (nibble(b[2 * i]) << 4) | nibble(b[2 * i + 1]);
    }
    out
}

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => panic!("bad hex digit"),
    }
}

#[test]
fn sha3_256_empty() {
    assert_eq!(
        sha3_256(b""),
        hex32("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a")
    );
}

#[test]
fn sha3_256_abc() {
    assert_eq!(
        sha3_256(b"abc"),
        hex32("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532")
    );
}

#[test]
fn sha3_512_empty() {
    assert_eq!(
        sha3_512(b""),
        hex64(
            "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a6\
             15b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"
        )
    );
}

#[test]
fn sha3_512_abc() {
    assert_eq!(
        sha3_512(b"abc"),
        hex64(
            "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e\
             10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"
        )
    );
}

#[test]
fn sha3_is_deterministic_and_avalanches() {
    assert_eq!(sha3_256(b"nonos"), sha3_256(b"nonos"));
    assert_ne!(sha3_256(b"nonos"), sha3_256(b"nonoS"));
}
