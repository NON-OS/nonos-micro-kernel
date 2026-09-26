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


//! getdents64 records, which a libc walks by adding d_reclen.

use alloc::vec::Vec;

use crate::dirent::{encode, record_len, DT_UNKNOWN, HEADER};

#[test]
fn a_record_is_eight_byte_aligned_and_holds_the_terminator() {
    for name in ["a", "ab", "abc", "abcd", "abcde"] {
        let want = HEADER + name.len() + 1;
        let got = record_len(name.as_bytes());
        assert!(got >= want, "{name}: {got} must hold {want}");
        assert_eq!(got % 8, 0, "{name}: {got} is not aligned");
        assert!(got - want < 8, "{name}: {got} pads more than a word");
    }
}

#[test]
fn walking_by_reclen_reaches_every_name() {
    let names = ["bin", "lib", "usr", "a-much-longer-name"];
    let mut out: Vec<u8> = Vec::new();
    for (i, name) in names.iter().enumerate() {
        encode(&mut out, name.as_bytes(), i as u64, DT_UNKNOWN);
    }
    let mut at = 0usize;
    let mut seen = Vec::new();
    while at < out.len() {
        let reclen = u16::from_le_bytes([out[at + 16], out[at + 17]]) as usize;
        assert!(reclen >= HEADER + 1 && at + reclen <= out.len());
        let body = &out[at + 19..at + reclen];
        let end = body.iter().position(|b| *b == 0).unwrap();
        seen.push(String::from_utf8(body[..end].to_vec()).unwrap());
        at += reclen;
    }
    assert_eq!(seen, names);
}

#[test]
fn the_offset_field_advances_with_the_index() {
    let mut out: Vec<u8> = Vec::new();
    encode(&mut out, b"x", 7, DT_UNKNOWN);
    assert_eq!(u64::from_le_bytes(out[0..8].try_into().unwrap()), 8);
    assert_eq!(i64::from_le_bytes(out[8..16].try_into().unwrap()), 8);
}

#[test]
fn the_type_is_unknown_and_not_a_guess() {
    let mut out: Vec<u8> = Vec::new();
    encode(&mut out, b"x", 0, DT_UNKNOWN);
    assert_eq!(out[18], 0);
}
