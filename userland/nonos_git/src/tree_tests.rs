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

//! Trees and commits against the exact bytes and ids real git produced.
//!
//! The anchors come from a repository built with `git init`, `git add f.txt`
//! and a commit made with fixed author and committer dates, so the ids here are
//! the ids git computed for that content. Reproducing them means a tree or
//! commit written by the terminal is one git will accept.

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::commit::{encode as encode_commit, parse as parse_commit, Commit, Signature};
use crate::object::{frame, ObjectKind};
use crate::oid::ObjectId;
use crate::tree::{encode as encode_tree, parse as parse_tree, Mode, TreeEntry, TreeError};

const BLOB_HELLO: &str = "ce013625030ba8dba906f756967f9e9ca394464a";
const TREE_ID: &str = "b4ed918248039b78f24383523fa4e51f80994fac";
const COMMIT_ID: &str = "08b165d07082a80c681c8c0217a4e45b1c595747";

fn entry(name: &str, mode: Mode, id_hex: &str) -> TreeEntry {
    TreeEntry { mode, name: String::from(name), id: ObjectId::from_hex(id_hex).expect("hex id") }
}

#[test]
fn a_tree_matches_the_id_git_computed() {
    // `git add f.txt && git write-tree` on a file holding "hello\n".
    let mut entries = vec![entry("f.txt", Mode::File, BLOB_HELLO)];
    let content = encode_tree(&mut entries);
    // The entry is the octal mode, a space, the name, a NUL, then the id raw.
    assert_eq!(&content[..14], b"100644 f.txt\x00\xce");
    let (_framed, id) = frame(ObjectKind::Tree, &content);
    assert_eq!(id.to_hex(), TREE_ID);
}

#[test]
fn a_commit_matches_the_id_git_computed() {
    let sig = Signature {
        name: String::from("ek"),
        email: String::from("ek@nonos.systems"),
        when: 1_700_000_000,
        offset_minutes: 0,
    };
    let commit = Commit {
        tree: ObjectId::from_hex(TREE_ID).unwrap(),
        parents: Vec::new(),
        author: sig.clone(),
        committer: sig,
        message: String::from("first\n"),
    };
    let content = encode_commit(&commit);
    assert_eq!(
        core::str::from_utf8(&content).unwrap(),
        "tree b4ed918248039b78f24383523fa4e51f80994fac\n\
         author ek <ek@nonos.systems> 1700000000 +0000\n\
         committer ek <ek@nonos.systems> 1700000000 +0000\n\
         \n\
         first\n"
    );
    let (_framed, id) = frame(ObjectKind::Commit, &content);
    assert_eq!(id.to_hex(), COMMIT_ID);
}

#[test]
fn a_commit_round_trips() {
    let sig = Signature {
        name: String::from("ek"),
        email: String::from("ek@nonos.systems"),
        when: 1_700_000_000,
        offset_minutes: -300,
    };
    let commit = Commit {
        tree: ObjectId::from_hex(TREE_ID).unwrap(),
        parents: vec![
            ObjectId::from_hex(COMMIT_ID).unwrap(),
            ObjectId::from_hex(BLOB_HELLO).unwrap(),
        ],
        author: sig.clone(),
        committer: sig,
        message: String::from("a merge\n\nwith a body\n"),
    };
    let content = encode_commit(&commit);
    let back = parse_commit(&content).expect("round trip");
    assert_eq!(back, commit);
    // A negative zone offset survives the trip verbatim, since it is hashed.
    assert_eq!(back.author.offset_minutes, -300);
    assert_eq!(back.parents.len(), 2);
}

#[test]
fn a_tree_round_trips() {
    let mut entries = vec![
        entry("z.txt", Mode::File, BLOB_HELLO),
        entry("dir", Mode::Directory, TREE_ID),
        entry("run.sh", Mode::Executable, BLOB_HELLO),
        entry("link", Mode::Symlink, BLOB_HELLO),
    ];
    let content = encode_tree(&mut entries);
    let back = parse_tree(&content).expect("round trip");
    assert_eq!(back, entries);
}

#[test]
fn directories_sort_as_if_they_ended_in_a_slash() {
    // The rule that decides the hash: `foo.txt` sorts before the directory
    // `foo`, because git compares `foo.` against `foo/` and `.` is 0x2E while
    // `/` is 0x2F. Sorting the plain names would put `foo` first and produce a
    // different, wrong tree id.
    let mut entries =
        vec![entry("foo", Mode::Directory, TREE_ID), entry("foo.txt", Mode::File, BLOB_HELLO)];
    let content = encode_tree(&mut entries);
    assert_eq!(entries[0].name, "foo.txt");
    assert_eq!(entries[1].name, "foo");
    // And the encoding reflects that order.
    assert!(content.starts_with(b"100644 foo.txt\x00"));
}

#[test]
fn entries_sort_by_raw_bytes() {
    let mut entries = vec![
        entry("b", Mode::File, BLOB_HELLO),
        entry("A", Mode::File, BLOB_HELLO),
        entry("a", Mode::File, BLOB_HELLO),
    ];
    encode_tree(&mut entries);
    // Uppercase sorts before lowercase because the comparison is on bytes.
    assert_eq!(entries[0].name, "A");
    assert_eq!(entries[1].name, "a");
    assert_eq!(entries[2].name, "b");
}

#[test]
fn a_corrupt_tree_is_rejected() {
    // Truncated id.
    let mut short: Vec<u8> = Vec::new();
    short.extend_from_slice(b"100644 f.txt\x00");
    short.extend_from_slice(&[0u8; 10]);
    assert_eq!(parse_tree(&short), Err(TreeError::Truncated));

    // Mode git would never write.
    let mut bad_mode: Vec<u8> = Vec::new();
    bad_mode.extend_from_slice(b"100777 f.txt\x00");
    bad_mode.extend_from_slice(&[0u8; 20]);
    assert_eq!(parse_tree(&bad_mode), Err(TreeError::Mode));
}

#[test]
fn a_tree_naming_a_parent_directory_is_rejected() {
    // A name of `..` would let a checkout escape the work tree.
    let mut escape: Vec<u8> = Vec::new();
    escape.extend_from_slice(b"100644 ..\x00");
    escape.extend_from_slice(&[0u8; 20]);
    assert_eq!(parse_tree(&escape), Err(TreeError::Name));

    // So would an embedded slash.
    let mut slash: Vec<u8> = Vec::new();
    slash.extend_from_slice(b"100644 a/b\x00");
    slash.extend_from_slice(&[0u8; 20]);
    assert_eq!(parse_tree(&slash), Err(TreeError::Name));
}

#[test]
fn an_out_of_order_tree_is_rejected() {
    // Hand-built with `b` before `a`: git would never write this, and accepting
    // it would mean re-encoding produces a different id than it parsed from.
    let raw_id = [0u8; 20];
    let mut out: Vec<u8> = Vec::new();
    for name in [b"b".as_slice(), b"a".as_slice()] {
        out.extend_from_slice(b"100644 ");
        out.extend_from_slice(name);
        out.push(0);
        out.extend_from_slice(&raw_id);
    }
    assert_eq!(parse_tree(&out), Err(TreeError::Order));
}
