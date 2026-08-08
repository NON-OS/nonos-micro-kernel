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

//! Trees and commits against the ids real git computed for the same content.

use nonos_git::{
    encode_commit, encode_tree, frame, parse_commit, Commit, Mode, ObjectId, ObjectKind, Signature,
    TreeEntry,
};

const BLOB: &str = "ce013625030ba8dba906f756967f9e9ca394464a";
const TREE: &str = "b4ed918248039b78f24383523fa4e51f80994fac";
const COMMIT: &str = "08b165d07082a80c681c8c0217a4e45b1c595747";

fn entry(name: &str, mode: Mode, id_hex: &str) -> TreeEntry {
    TreeEntry { mode, name: String::from(name), id: ObjectId::from_hex(id_hex).expect("hex") }
}

fn sig(offset_minutes: i16) -> Signature {
    Signature {
        name: String::from("ek"),
        email: String::from("ek@nonos.systems"),
        when: 1_700_000_000,
        offset_minutes,
    }
}

#[test]
fn a_tree_matches_the_id_git_computed() {
    let mut entries = vec![entry("f.txt", Mode::File, BLOB)];
    let content = encode_tree(&mut entries);
    // Octal mode, space, name, NUL, then the id raw rather than hex.
    assert_eq!(&content[..14], b"100644 f.txt\x00\xce");
    assert_eq!(frame(ObjectKind::Tree, &content).1.to_hex(), TREE);
}

#[test]
fn a_commit_matches_the_id_git_computed() {
    let commit = Commit {
        tree: ObjectId::from_hex(TREE).unwrap(),
        parents: Vec::new(),
        author: sig(0),
        committer: sig(0),
        message: String::from("first\n"),
    };
    let content = encode_commit(&commit);
    assert_eq!(frame(ObjectKind::Commit, &content).1.to_hex(), COMMIT);
}

#[test]
fn a_commit_round_trips_with_parents_and_a_negative_offset() {
    let commit = Commit {
        tree: ObjectId::from_hex(TREE).unwrap(),
        parents: vec![ObjectId::from_hex(COMMIT).unwrap(), ObjectId::from_hex(BLOB).unwrap()],
        author: sig(-300),
        committer: sig(-300),
        message: String::from("a merge\n\nwith a body\n"),
    };
    assert_eq!(parse_commit(&encode_commit(&commit)).expect("round trip"), commit);
}
