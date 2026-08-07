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

//! The object database over a real directory.

mod common;

use common::Scratch;

use nonos_git::{
    compress, frame, init, read_object, write_object, ObjectId, ObjectKind, OdbError, Storage,
};

#[test]
fn an_object_round_trips() {
    let scratch = Scratch::new("odb");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");

    let id = write_object(&mut storage, ".git", ObjectKind::Blob, b"hello\n").expect("write");
    assert_eq!(id.to_hex(), "ce013625030ba8dba906f756967f9e9ca394464a");
    assert!(storage.exists(".git/objects/ce/013625030ba8dba906f756967f9e9ca394464a"));
    let got = read_object(&storage, ".git", &id).expect("read");
    assert_eq!(got, (ObjectKind::Blob, b"hello\n".to_vec()));
}

#[test]
fn writing_the_same_object_twice_is_stable() {
    let scratch = Scratch::new("twice");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    let a = write_object(&mut storage, ".git", ObjectKind::Blob, b"same\n").expect("a");
    let b = write_object(&mut storage, ".git", ObjectKind::Blob, b"same\n").expect("b");
    assert_eq!(a, b);
}

#[test]
fn a_missing_object_is_not_found() {
    let scratch = Scratch::new("missing");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    let absent = ObjectId::from_hex("0123456789012345678901234567890123456789").unwrap();
    assert_eq!(read_object(&storage, ".git", &absent), Err(OdbError::NotFound));
}

#[test]
fn a_damaged_object_is_caught_rather_than_returned() {
    let scratch = Scratch::new("damaged");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    let id = write_object(&mut storage, ".git", ObjectKind::Blob, b"hello\n").expect("write");

    // A valid stream holding different content: it inflates, but no longer
    // hashes to the id it sits under.
    let (framed, _other) = frame(ObjectKind::Blob, b"tampered\n");
    let path = ".git/objects/ce/013625030ba8dba906f756967f9e9ca394464a";
    storage.write(path, &compress(&framed)).expect("overwrite");
    assert_eq!(read_object(&storage, ".git", &id), Err(OdbError::IdMismatch));
}
