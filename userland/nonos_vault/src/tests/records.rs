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

//! A record round-trips, and every byte of it is under the tag.

use crate::blob::{MAGIC, VERSION};
use crate::open;

use super::*;

#[test]
fn a_record_opens_to_what_was_sealed() {
    for len in [1usize, 15, 32, 72, 300] {
        let plaintext: alloc::vec::Vec<u8> = (0..len).map(|i| (i * 7) as u8).collect();
        let blob = sealed(b"wallet.account", &plaintext);
        let mut out = alloc::vec![0u8; len];
        let n = open(&ROOT, b"wallet.account", &blob, &mut out).expect("open");
        assert_eq!(n, len);
        assert_eq!(out, plaintext);
    }
}

#[test]
fn every_byte_of_the_record_is_covered() {
    let blob = sealed(b"settings.theme", b"dark");
    for i in 0..blob.len() {
        let mut bad = blob.clone();
        bad[i] ^= 0x01;
        let mut out = [0u8; 4];
        assert!(
            open(&ROOT, b"settings.theme", &bad, &mut out).is_err(),
            "a flip at byte {i} was accepted"
        );
    }
}

#[test]
fn the_header_says_what_it_is() {
    let blob = sealed(b"settings.theme", b"dark");
    assert_eq!(&blob[..8], &MAGIC);
    assert_eq!(u16::from_le_bytes([blob[8], blob[9]]), VERSION);
    assert_eq!(blob[10], b"settings.theme".len() as u8);
    assert_eq!(blob[11], 0, "the reserved byte stays zero");
}
