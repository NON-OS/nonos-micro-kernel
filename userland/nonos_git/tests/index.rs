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

//! Staging into the index.

mod common;

use common::Scratch;

use nonos_git::{add, read_index, Mode, Storage};

#[test]
fn adding_a_file_stages_it_and_the_index_round_trips() {
    let scratch = Scratch::new("stage_one");
    let mut storage = scratch.storage();
    nonos_git::init(&mut storage, ".git", "main").expect("init");
    storage.write("f.txt", b"hello\n").expect("work file");

    let blob = add(&mut storage, ".git", "f.txt", Mode::File).expect("add");
    assert_eq!(blob.to_hex(), "ce013625030ba8dba906f756967f9e9ca394464a");

    let entries = read_index(&storage, ".git").expect("read index");
    assert_eq!(entries.len(), 1);
    assert_eq!((entries[0].path.as_str(), entries[0].id, entries[0].size), ("f.txt", blob, 6));
}

#[test]
fn staging_a_path_twice_replaces_its_entry() {
    let scratch = Scratch::new("restage");
    let mut storage = scratch.storage();
    nonos_git::init(&mut storage, ".git", "main").expect("init");

    storage.write("f.txt", b"first\n").expect("work file");
    add(&mut storage, ".git", "f.txt", Mode::File).expect("add");
    storage.write("f.txt", b"second\n").expect("work file");
    let second = add(&mut storage, ".git", "f.txt", Mode::File).expect("re-add");

    let entries = read_index(&storage, ".git").expect("read index");
    assert_eq!(entries.len(), 1, "the path must appear once");
    assert_eq!(entries[0].id, second);
}

#[test]
fn the_index_stays_sorted_by_path() {
    let scratch = Scratch::new("sorted");
    let mut storage = scratch.storage();
    nonos_git::init(&mut storage, ".git", "main").expect("init");
    for name in ["z.txt", "a.txt", "m.txt"] {
        storage.write(name, b"x\n").expect("work file");
        add(&mut storage, ".git", name, Mode::File).expect("add");
    }

    let entries = read_index(&storage, ".git").expect("read index");
    let paths: Vec<&str> = entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(paths, ["a.txt", "m.txt", "z.txt"]);
}
