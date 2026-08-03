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
//! A pack and index we wrote, read by real git.
//!
//! The objects in this repository exist only inside the pack. If git can
//! resolve them, the pack and the index are both right, because there is
//! nowhere else for the bytes to come from.

mod common;

use common::{git, git_available, Scratch};

use nonos_git::{index_entries, pack_lookup, read_pack, write_pack_index, ObjectId, Sha1, Storage};

const PACK: &[u8] = include_bytes!("data/simple.pack");
const COMMIT: &str = "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d";
const BLOB: &str = "980a0d5f19a64b4b30a87d4206aade58726b60e3";

#[test]
fn an_index_finds_every_object_in_its_pack() {
    let objects = read_pack(PACK).expect("pack");
    let rows = index_entries(PACK, &objects).expect("rows");
    let sha = Sha1::digest(&PACK[..PACK.len() - 20]);
    let idx = write_pack_index(&rows, &sha).expect("index");

    for object in &objects {
        let at = pack_lookup(&idx, &object.id).expect("id must be in the index");
        assert_eq!(at, object.offset as u64);
    }
    // An id the pack does not hold is absent rather than a wrong offset.
    let absent = ObjectId::from_bytes([0x11; 20]);
    assert_eq!(pack_lookup(&idx, &absent), None);
}

#[test]
fn git_resolves_objects_that_exist_only_in_our_pack() {
    if !git_available() {
        return;
    }
    let scratch = Scratch::new("packstore");
    let mut storage = scratch.storage();
    nonos_git::init(&mut storage, ".git", "master").expect("init");

    let objects = read_pack(PACK).expect("pack");
    let rows = index_entries(PACK, &objects).expect("rows");
    let sha = Sha1::digest(&PACK[..PACK.len() - 20]);
    let idx = write_pack_index(&rows, &sha).expect("index");

    let name = format!(".git/objects/pack/pack-{}", ObjectId::from_bytes(sha).to_hex());
    storage.write(&format!("{name}.pack"), PACK).expect("pack");
    storage.write(&format!("{name}.idx"), &idx).expect("idx");

    // Nothing loose was written, so every answer below comes from the pack.
    git(&scratch.path, &["verify-pack", "-v", &format!("{name}.idx")]);
    assert_eq!(git(&scratch.path, &["cat-file", "-t", COMMIT]).trim(), "commit");
    assert_eq!(git(&scratch.path, &["cat-file", "blob", BLOB]), "Hello World!\n");
    git(&scratch.path, &["fsck", "--strict", "--no-dangling"]);
}
