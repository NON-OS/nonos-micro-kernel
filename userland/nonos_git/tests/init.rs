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

//! Creating a repository.

mod common;

use common::Scratch;

use nonos_git::{init, read_head, resolve_head, update_ref, Head, ObjectId, RepoError, Storage};

#[test]
fn init_creates_a_repository_git_would_recognise() {
    let scratch = Scratch::new("init");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");

    assert!(storage.is_dir(".git/objects"));
    assert!(storage.is_dir(".git/refs/heads"));
    assert_eq!(storage.read(".git/HEAD").unwrap(), b"ref: refs/heads/main\n");
    // A fresh repository is on an unborn branch: named, but with no commit.
    assert_eq!(read_head(&storage, ".git"), Some(Head::Branch(String::from("main"))));
    assert_eq!(resolve_head(&storage, ".git"), None);
}

#[test]
fn init_refuses_to_overwrite_a_repository() {
    let scratch = Scratch::new("reinit");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    assert_eq!(init(&mut storage, ".git", "main"), Err(RepoError::Exists));
}

#[test]
fn a_branch_name_that_would_escape_refs_is_refused() {
    let scratch = Scratch::new("badref");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    let id = ObjectId::from_hex("0123456789012345678901234567890123456789").unwrap();
    for bad in ["../../evil", "/abs", "a..b", "with space", "trailing/", "x.lock", ".hidden"] {
        assert!(update_ref(&mut storage, ".git", bad, &id).is_err(), "{bad} must be refused");
    }
    assert!(update_ref(&mut storage, ".git", "feature/work", &id).is_ok());
}
