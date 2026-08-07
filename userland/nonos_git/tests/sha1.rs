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

//! SHA-1 against the standard vectors and the block-boundary sizes.

use nonos_git::Sha1;

fn hex(bytes: &[u8]) -> String {
    let mut s = String::new();
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0xF) as u32, 16).unwrap());
    }
    s
}

#[test]
fn matches_the_standard_vectors() {
    assert_eq!(hex(&Sha1::digest(b"")), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    assert_eq!(hex(&Sha1::digest(b"abc")), "a9993e364706816aba3e25717850c26c9cd0d89d");
    assert_eq!(
        hex(&Sha1::digest(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")),
        "84983e441c3bd26ebaae4aa1f95129e5e54670f1"
    );
    assert_eq!(hex(&Sha1::digest(&vec![b'a'; 1000])), "291e9a6c66994949b57ba5e650361e98fc36b1ba");
}

#[test]
fn streaming_matches_one_shot_across_block_boundaries() {
    // 55 is the last that fits with padding in one block, 56 the first that
    // forces a second: where the padding arithmetic goes wrong if it does.
    for n in [54usize, 55, 56, 63, 64, 65, 119, 120, 121, 1000] {
        let data = vec![b'a'; n];
        let mut h = Sha1::new();
        h.update(&data[..n / 2]);
        h.update(&data[n / 2..]);
        assert_eq!(Sha1::digest(&data), h.finish(), "n={n}");
    }
}
