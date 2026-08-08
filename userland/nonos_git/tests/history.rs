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

//! Commits and history.

mod common;

use common::{build_repo, signature, Scratch};

use nonos_git::{
    commit, encode_tree, log, resolve_head, write_object, CommitRequest, Mode, ObjectKind, Storage,
    TreeEntry,
};

#[test]
fn a_commit_moves_the_branch_and_shows_up_in_the_log() {
    let scratch = Scratch::new("commit");
    let mut storage = scratch.storage();
    let (id, _blob) = build_repo(&mut storage);

    let branch = storage.read(".git/refs/heads/main").unwrap();
    assert_eq!(std::str::from_utf8(&branch).unwrap().trim(), id.to_hex());
    assert_eq!(resolve_head(&storage, ".git"), Some(id));

    let entries = log(&storage, ".git", 10).expect("log");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].commit.message, "first\n");
    // A first commit is a root commit.
    assert!(entries[0].commit.parents.is_empty());
}

#[test]
fn a_second_commit_continues_the_branch() {
    let scratch = Scratch::new("second");
    let mut storage = scratch.storage();
    let (first, _blob) = build_repo(&mut storage);

    let blob = write_object(&mut storage, ".git", ObjectKind::Blob, b"second\n").expect("blob");
    let mut entries = vec![TreeEntry { mode: Mode::File, name: String::from("g.txt"), id: blob }];
    let bytes = encode_tree(&mut entries);
    let tree = write_object(&mut storage, ".git", ObjectKind::Tree, &bytes).expect("tree");
    let request = CommitRequest {
        tree,
        author: signature(),
        committer: signature(),
        message: String::from("second\n"),
    };
    let second = commit(&mut storage, ".git", &request).expect("commit");

    let entries = log(&storage, ".git", 10).expect("log");
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].id, second);
    assert_eq!(entries[0].commit.parents, vec![first]);
}

#[test]
fn the_log_is_bounded_by_its_limit() {
    let scratch = Scratch::new("limit");
    let mut storage = scratch.storage();
    build_repo(&mut storage);
    assert_eq!(log(&storage, ".git", 0).expect("log").len(), 0);
}
