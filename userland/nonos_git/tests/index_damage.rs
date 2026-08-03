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
//! A damaged index is refused rather than partly believed.

mod common;

use common::Scratch;

use nonos_git::{add, init, read_index, IndexError, Mode, RepoError, Storage};

#[test]
fn a_damaged_index_is_refused() {
    let scratch = Scratch::new("bad_index");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    storage.write("f.txt", b"hello\n").expect("work file");
    add(&mut storage, ".git", "f.txt", Mode::File).expect("add");

    // Flip a byte in the body: the trailing checksum no longer covers it.
    let mut raw = storage.read(".git/index").expect("index");
    raw[20] ^= 0xFF;
    storage.write(".git/index", &raw).expect("overwrite");
    assert_eq!(read_index(&storage, ".git"), Err(RepoError::Index(IndexError::Checksum)));

    // Long enough to pass the length check, so the magic is what refuses it.
    let not_index = b"XXXX and then thirty more bytes of padding to be long enough";
    storage.write(".git/index", not_index).expect("overwrite");
    assert_eq!(read_index(&storage, ".git"), Err(RepoError::Index(IndexError::Magic)));

    // And a file too short to hold a header at all.
    storage.write(".git/index", b"DIRC").expect("overwrite");
    assert_eq!(read_index(&storage, ".git"), Err(RepoError::Index(IndexError::Truncated)));
}
