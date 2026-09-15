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

//! Who may hand a file to the block store: the pid that created it, as the
//! kernel stamped it, and nobody else.

use crate::vfs_store::{Store, StoreError};

const MINE: u32 = 7;
const THEIRS: u32 = 8;

fn put(store: &mut Store, path: &str, pid: u32, data: &[u8]) {
    let fd = store.open(path, pid, true, true, false, true).expect("open create");
    store.write(fd, pid, data).expect("write");
    store.close(fd, pid).expect("close");
}

#[test]
fn the_creator_may_persist_its_file_and_nobody_else_may() {
    let mut s = Store::new();
    put(&mut s, "/data/wallet.vault", MINE, b"sealed");
    assert_eq!(s.persistable("/data/wallet.vault", MINE), Ok(b"sealed".to_vec()));
    assert_eq!(s.persistable("/data/wallet.vault", THEIRS), Err(StoreError::AccessDenied));
    assert_eq!(s.persistable("/data/absent", MINE), Err(StoreError::NotFound));
}

#[test]
fn a_copy_belongs_to_whoever_copied_it() {
    let mut s = Store::new();
    put(&mut s, "/data/a", MINE, b"x");
    s.copy("/data/a", "/data/b", false, THEIRS).expect("copied");
    assert!(s.persistable("/data/b", THEIRS).is_ok());
    assert_eq!(s.persistable("/data/b", MINE), Err(StoreError::AccessDenied));
    assert!(s.persistable("/data/a", MINE).is_ok(), "the original keeps its owner");
}

#[test]
fn rewriting_a_file_does_not_change_who_owns_it() {
    let mut s = Store::new();
    put(&mut s, "/data/a", MINE, b"one");
    put(&mut s, "/data/a", MINE, b"two");
    assert_eq!(s.persistable("/data/a", MINE), Ok(b"two".to_vec()));
}
