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

//! The tree sort rule and the refusals that keep a tree sound.

use nonos_git::{encode_tree, parse_tree, Mode, ObjectId, TreeEntry, TreeError};

const BLOB: &str = "ce013625030ba8dba906f756967f9e9ca394464a";
const TREE: &str = "b4ed918248039b78f24383523fa4e51f80994fac";

fn entry(name: &str, mode: Mode, id_hex: &str) -> TreeEntry {
    TreeEntry { mode, name: String::from(name), id: ObjectId::from_hex(id_hex).expect("hex") }
}

#[test]
fn directories_sort_as_if_they_ended_in_a_slash() {
    // The rule that decides the hash: git compares `foo.` against `foo/`, and
    // `.` is 0x2E while `/` is 0x2F, so the file sorts first. Sorting the plain
    // names would give a different, wrong tree id.
    let mut entries = vec![entry("foo", Mode::Directory, TREE), entry("foo.txt", Mode::File, BLOB)];
    let content = encode_tree(&mut entries);
    assert_eq!(entries[0].name, "foo.txt");
    assert!(content.starts_with(b"100644 foo.txt\x00"));
}

#[test]
fn entries_sort_by_raw_bytes() {
    let mut entries = vec![
        entry("b", Mode::File, BLOB),
        entry("A", Mode::File, BLOB),
        entry("a", Mode::File, BLOB),
    ];
    encode_tree(&mut entries);
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["A", "a", "b"]);
}

/// A hand-built entry, for cases the encoder would never produce.
fn raw(mode: &[u8], name: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(mode);
    out.push(b' ');
    out.extend_from_slice(name);
    out.push(0);
    out.extend_from_slice(&[0u8; 20]);
    out
}

#[test]
fn a_corrupt_tree_is_rejected() {
    let mut short = raw(b"100644", b"f.txt");
    short.truncate(short.len() - 10);
    assert_eq!(parse_tree(&short), Err(TreeError::Truncated));
    assert_eq!(parse_tree(&raw(b"100777", b"f.txt")), Err(TreeError::Mode));
}

#[test]
fn a_name_that_could_escape_the_work_tree_is_rejected() {
    assert_eq!(parse_tree(&raw(b"100644", b"..")), Err(TreeError::Name));
    assert_eq!(parse_tree(&raw(b"100644", b"a/b")), Err(TreeError::Name));
}
