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
//! Cloning a real pack into a repository, then handing it to git.
//!
//! `simple.pack` is what GitHub sends for a depth-1 fetch of
//! octocat/Hello-World: one commit, one tree, one blob.

mod common;

use common::{git, git_available, Scratch};

use nonos_git::{clone_into, CloneRequest, ObjectId, Storage};

const PACK: &[u8] = include_bytes!("data/simple.pack");
const HEAD: &str = "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d";

#[test]
fn a_fetched_pack_becomes_a_working_repository() {
    let scratch = Scratch::new("clone");
    let mut storage = scratch.storage();
    let head = ObjectId::from_hex(HEAD).expect("head id");

    let request =
        CloneRequest { git_dir: ".git", work_tree: "", head, branch: "master", shallow: true };
    let written = clone_into(&mut storage, &request, PACK).expect("clone");
    assert_eq!(written, 1);

    let readme = storage.read("README").expect("checked out file");
    assert_eq!(readme, b"Hello World!\n");
}

#[test]
fn git_agrees_the_clone_is_intact() {
    if !git_available() {
        return;
    }
    let scratch = Scratch::new("clone_git");
    let mut storage = scratch.storage();
    let head = ObjectId::from_hex(HEAD).expect("head id");
    let request =
        CloneRequest { git_dir: ".git", work_tree: "", head, branch: "master", shallow: true };
    clone_into(&mut storage, &request, PACK).expect("clone");

    git(&scratch.path, &["fsck", "--strict"]);
    assert_eq!(git(&scratch.path, &["rev-parse", "HEAD"]).trim(), HEAD);
    // Nothing to report means the work tree matches the commit git just read.
    let status = git(&scratch.path, &["status", "--porcelain"]);
    assert!(status.trim().is_empty(), "unexpected status: {status}");
}
