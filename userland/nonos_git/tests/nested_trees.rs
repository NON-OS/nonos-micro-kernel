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

//! Nested directories built from the flat index.

mod common;

use common::Scratch;

use nonos_git::{
    add, init, parse_tree, read_index, read_object, write_tree, Mode, ObjectKind, Storage,
};

#[test]
fn a_nested_path_becomes_nested_trees() {
    let scratch = Scratch::new("nested");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");

    storage.write("top.txt", b"top\n").expect("work file");
    storage.write("dir/inner.txt", b"inner\n").expect("work file");
    storage.write("dir/deep/leaf.txt", b"leaf\n").expect("work file");
    for p in ["top.txt", "dir/inner.txt", "dir/deep/leaf.txt"] {
        add(&mut storage, ".git", p, Mode::File).expect("add");
    }

    let entries = read_index(&storage, ".git").expect("read index");
    assert_eq!(entries.len(), 3);

    // The root holds one file and one directory, not three flat paths.
    let root = write_tree(&mut storage, ".git", &entries).expect("write tree");
    let (kind, content) = read_object(&storage, ".git", &root).expect("read root");
    assert_eq!(kind, ObjectKind::Tree);
    let parsed = parse_tree(&content).expect("parse root");
    let names: Vec<&str> = parsed.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["dir", "top.txt"]);
    assert_eq!(parsed[0].mode, Mode::Directory);
}
