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

//! The hash and the object identity, checked against what real git produces.
//!
//! The object-id cases are the exact hashes `git hash-object` prints for the
//! same content, so passing them means a blob written here is a blob git can
//! read. The SHA-1 cases are the FIPS 180-4 and de-facto vectors, including
//! inputs that cross the 64-byte block boundary the padding turns on.

use crate::object::{frame, unframe, ObjectKind};
use crate::oid::ObjectId;
use crate::sha1::Sha1;

extern crate alloc;
use alloc::string::String;
use alloc::vec;

fn hex(bytes: &[u8]) -> String {
    let mut s = String::new();
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0xF) as u32, 16).unwrap());
    }
    s
}

#[test]
fn sha1_matches_the_standard_vectors() {
    assert_eq!(hex(&Sha1::digest(b"")), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    assert_eq!(hex(&Sha1::digest(b"abc")), "a9993e364706816aba3e25717850c26c9cd0d89d");
    assert_eq!(
        hex(&Sha1::digest(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")),
        "84983e441c3bd26ebaae4aa1f95129e5e54670f1"
    );
}

#[test]
fn sha1_handles_block_boundaries() {
    // 55 bytes: the last that fits with padding in one block. 56: the first
    // that forces a second block. 64: exactly one block. These are where the
    // padding arithmetic goes wrong if it is wrong at all.
    for n in [54usize, 55, 56, 63, 64, 65, 119, 120, 121, 1000] {
        let data = vec![b'a'; n];
        // Compare the streaming path against a fresh one-shot over the same
        // bytes fed in two pieces, so a bug in `update` chunking shows up.
        let one = Sha1::digest(&data);
        let mut h = Sha1::new();
        h.update(&data[..n / 2]);
        h.update(&data[n / 2..]);
        assert_eq!(one, h.finish(), "n={n}");
    }
    // A known million-'a' style anchor at a smaller, checkable size.
    assert_eq!(hex(&Sha1::digest(&vec![b'a'; 1000])), "291e9a6c66994949b57ba5e650361e98fc36b1ba");
}

#[test]
fn a_blob_id_matches_git() {
    // `printf 'hello\n' | git hash-object --stdin`
    let (_framed, id) = frame(ObjectKind::Blob, b"hello\n");
    assert_eq!(id.to_hex(), "ce013625030ba8dba906f756967f9e9ca394464a");
}

#[test]
fn the_empty_blob_id_matches_git() {
    // The well-known empty blob id.
    let (_framed, id) = frame(ObjectKind::Blob, b"");
    assert_eq!(id.to_hex(), "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");
}

#[test]
fn framing_carries_the_header_git_expects() {
    let (framed, _id) = frame(ObjectKind::Blob, b"hello\n");
    assert_eq!(&framed, b"blob 6\0hello\n");
    let (framed, _id) = frame(ObjectKind::Commit, b"");
    assert_eq!(&framed, b"commit 0\0");
}

#[test]
fn framing_round_trips_through_unframe() {
    for kind in [ObjectKind::Blob, ObjectKind::Tree, ObjectKind::Commit, ObjectKind::Tag] {
        let content = b"some bytes\x00with a nul and more";
        let (framed, _id) = frame(kind, content);
        let (back_kind, back_content) = unframe(&framed).expect("valid frame");
        assert_eq!(back_kind, kind);
        assert_eq!(back_content, content);
    }
}

#[test]
fn a_corrupt_frame_is_rejected() {
    // Length that disagrees with the content.
    assert!(unframe(b"blob 99\0short").is_none());
    // Unknown type.
    assert!(unframe(b"widget 3\0abc").is_none());
    // No NUL terminator.
    assert!(unframe(b"blob 3 abc").is_none());
}

#[test]
fn object_ids_round_trip_through_hex() {
    let (_framed, id) = frame(ObjectKind::Blob, b"hello\n");
    let hex = id.to_hex();
    let back = ObjectId::from_hex(&hex).expect("valid hex");
    assert_eq!(back, id);
    // The loose-object split is the two-char dir git uses.
    let (dir, file) = id.loose_path();
    assert_eq!(dir, "ce");
    assert_eq!(file.len(), 38);
    assert_eq!(dir.len() + file.len(), 40);
}

#[test]
fn malformed_hex_is_rejected() {
    assert!(ObjectId::from_hex("").is_none());
    assert!(ObjectId::from_hex("abc").is_none());
    assert!(ObjectId::from_hex(&"z".repeat(40)).is_none());
    // 39 and 41 chars, the off-by-ones.
    assert!(ObjectId::from_hex(&"a".repeat(39)).is_none());
    assert!(ObjectId::from_hex(&"a".repeat(41)).is_none());
}
