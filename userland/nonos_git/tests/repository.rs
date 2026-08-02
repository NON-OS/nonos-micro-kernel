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

//! The repository path over a real directory: init, store objects, commit,
//! walk the log, and the refusals that keep a repository sound.

mod common;

use common::Scratch;

use nonos_git::{
    commit, compress, encode_tree, frame, init, log, read_object, resolve_head, update_ref,
    write_object, CommitRequest, Head, Mode, ObjectId, ObjectKind, OdbError, RepoError, Signature,
    Storage, TreeEntry,
};

fn signature() -> Signature {
    Signature {
        name: String::from("ek"),
        email: String::from("ek@nonos.systems"),
        when: 1_700_000_000,
        offset_minutes: 0,
    }
}

/// Build a repository holding one file, returning the commit and blob ids.
fn build_repo(storage: &mut common::DirStorage) -> (ObjectId, ObjectId) {
    init(storage, ".git", "main").expect("init");
    let blob = write_object(storage, ".git", ObjectKind::Blob, b"hello\n").expect("blob");
    let mut entries = vec![TreeEntry { mode: Mode::File, name: String::from("f.txt"), id: blob }];
    let tree_bytes = encode_tree(&mut entries);
    let tree = write_object(storage, ".git", ObjectKind::Tree, &tree_bytes).expect("tree");
    let request = CommitRequest {
        tree,
        author: signature(),
        committer: signature(),
        message: String::from("first\n"),
    };
    (commit(storage, ".git", &request).expect("commit"), blob)
}

#[test]
fn init_creates_a_repository_git_would_recognise() {
    let scratch = Scratch::new("init");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");

    assert!(storage.exists(".git/HEAD"));
    assert!(storage.is_dir(".git/objects"));
    assert!(storage.is_dir(".git/refs/heads"));
    assert_eq!(storage.read(".git/HEAD").unwrap(), b"ref: refs/heads/main\n");
    // A fresh repository is on an unborn branch: HEAD names it, but no commit.
    assert_eq!(nonos_git::read_head(&storage, ".git"), Some(Head::Branch(String::from("main"))));
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
fn an_object_round_trips_through_the_database() {
    let scratch = Scratch::new("odb");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");

    let id = write_object(&mut storage, ".git", ObjectKind::Blob, b"hello\n").expect("write");
    // Stored under the id git gives that content, split two characters deep.
    assert_eq!(id.to_hex(), "ce013625030ba8dba906f756967f9e9ca394464a");
    assert!(storage.exists(".git/objects/ce/013625030ba8dba906f756967f9e9ca394464a"));

    let (kind, content) = read_object(&storage, ".git", &id).expect("read");
    assert_eq!(kind, ObjectKind::Blob);
    assert_eq!(content, b"hello\n");
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

    // Replace the stored bytes with a valid stream holding different content:
    // it inflates, but no longer hashes to the id it sits under.
    let (framed, _other) = frame(ObjectKind::Blob, b"tampered\n");
    let path = ".git/objects/ce/013625030ba8dba906f756967f9e9ca394464a";
    storage.write(path, &compress(&framed)).expect("overwrite");

    assert_eq!(read_object(&storage, ".git", &id), Err(OdbError::IdMismatch));
}

#[test]
fn a_commit_moves_the_branch_and_shows_up_in_the_log() {
    let scratch = Scratch::new("commit");
    let mut storage = scratch.storage();
    let (id, _blob) = build_repo(&mut storage);

    // The branch now names the commit, and HEAD resolves to it.
    let branch = storage.read(".git/refs/heads/main").unwrap();
    assert_eq!(std::str::from_utf8(&branch).unwrap().trim(), id.to_hex());
    assert_eq!(resolve_head(&storage, ".git"), Some(id));

    let entries = log(&storage, ".git", 10).expect("log");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id, id);
    assert_eq!(entries[0].commit.message, "first\n");
    // A first commit is a root commit: no parents.
    assert!(entries[0].commit.parents.is_empty());
}

#[test]
fn a_second_commit_continues_the_branch() {
    let scratch = Scratch::new("second");
    let mut storage = scratch.storage();
    let (first, _blob) = build_repo(&mut storage);

    let blob = write_object(&mut storage, ".git", ObjectKind::Blob, b"second\n").expect("blob");
    let mut entries = vec![TreeEntry { mode: Mode::File, name: String::from("g.txt"), id: blob }];
    let tree_bytes = encode_tree(&mut entries);
    let tree = write_object(&mut storage, ".git", ObjectKind::Tree, &tree_bytes).expect("tree");
    let request = CommitRequest {
        tree,
        author: signature(),
        committer: signature(),
        message: String::from("second\n"),
    };
    let second = commit(&mut storage, ".git", &request).expect("commit");

    let entries = log(&storage, ".git", 10).expect("log");
    assert_eq!(entries.len(), 2);
    // Newest first, and the second commit's parent is the first.
    assert_eq!(entries[0].id, second);
    assert_eq!(entries[1].id, first);
    assert_eq!(entries[0].commit.parents, vec![first]);
}

#[test]
fn the_log_is_bounded_by_its_limit() {
    let scratch = Scratch::new("limit");
    let mut storage = scratch.storage();
    build_repo(&mut storage);
    assert_eq!(log(&storage, ".git", 0).expect("log").len(), 0);
}

#[test]
fn writing_the_same_object_twice_is_stable() {
    let scratch = Scratch::new("twice");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    let a = write_object(&mut storage, ".git", ObjectKind::Blob, b"same\n").expect("first");
    let b = write_object(&mut storage, ".git", ObjectKind::Blob, b"same\n").expect("second");
    assert_eq!(a, b);
}

#[test]
fn a_branch_name_that_would_escape_refs_is_refused() {
    let scratch = Scratch::new("badref");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    let id = ObjectId::from_hex("0123456789012345678901234567890123456789").unwrap();
    // Each of these would write outside refs/heads if joined into a path.
    for bad in ["../../evil", "/abs", "a..b", "with space", "trailing/", "x.lock", ".hidden"] {
        assert!(update_ref(&mut storage, ".git", bad, &id).is_err(), "{bad} must be refused");
    }
    // An ordinary nested branch name is still allowed.
    assert!(update_ref(&mut storage, ".git", "feature/work", &id).is_ok());
}
