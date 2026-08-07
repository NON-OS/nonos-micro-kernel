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

//! Object framing and ids, against what real git computes.

use nonos_git::{frame, unframe, ObjectId, ObjectKind};

#[test]
fn blob_ids_match_git() {
    // `printf 'hello\n' | git hash-object --stdin`
    assert_eq!(
        frame(ObjectKind::Blob, b"hello\n").1.to_hex(),
        "ce013625030ba8dba906f756967f9e9ca394464a"
    );
    assert_eq!(frame(ObjectKind::Blob, b"").1.to_hex(), "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");
}

#[test]
fn framing_carries_the_header_git_expects() {
    assert_eq!(&frame(ObjectKind::Blob, b"hello\n").0, b"blob 6\0hello\n");
    assert_eq!(&frame(ObjectKind::Commit, b"").0, b"commit 0\0");
}

#[test]
fn framing_round_trips() {
    for kind in [ObjectKind::Blob, ObjectKind::Tree, ObjectKind::Commit, ObjectKind::Tag] {
        let content = b"some bytes\x00with a nul and more";
        let (framed, _id) = frame(kind, content);
        assert_eq!(unframe(&framed).expect("valid"), (kind, content.as_slice()));
    }
}

#[test]
fn a_corrupt_frame_is_rejected() {
    assert!(unframe(b"blob 99\0short").is_none());
    assert!(unframe(b"widget 3\0abc").is_none());
    assert!(unframe(b"blob 3 abc").is_none());
}

#[test]
fn ids_round_trip_through_hex() {
    let id = frame(ObjectKind::Blob, b"hello\n").1;
    assert_eq!(ObjectId::from_hex(&id.to_hex()).expect("hex"), id);
    let (dir, file) = id.loose_path();
    assert_eq!((dir.as_str(), file.len()), ("ce", 38));
}

#[test]
fn malformed_hex_is_rejected() {
    for bad in ["", "abc", &"z".repeat(40), &"a".repeat(39), &"a".repeat(41)] {
        assert!(ObjectId::from_hex(bad).is_none(), "{bad} must be refused");
    }
}
