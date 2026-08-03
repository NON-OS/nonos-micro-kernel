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
//! A clone driven end to end, over responses a real server sent.
//!
//! `advert_live.bin` is the service banner, the HEAD packet and the
//! refs/heads/master packet from github.com's advertisement for
//! octocat/Hello-World, reassembled without the other 3370 refs.
//! `live_uploadpack.bin` is what GitHub answered our depth-1 want request
//! with, shallow line and all.

mod common;

use common::{git, git_available, Replay, Scratch};

use nonos_git::{clone, Storage};

const ADVERT: &[u8] = include_bytes!("data/advert_live.bin");
const PACK: &[u8] = include_bytes!("data/live_uploadpack.bin");
const HEAD: &str = "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d";

#[test]
fn a_clone_asks_for_the_right_things_and_lands_a_repository() {
    let scratch = Scratch::new("remote_clone");
    let mut storage = scratch.storage();
    let mut transport = Replay::new(ADVERT, PACK);

    let files = clone(&mut transport, &mut storage, ".git", "", "master", 1).expect("clone");
    assert_eq!(files, 1);

    // Discovery first, then the pack, in that order and nothing else.
    assert_eq!(transport.asked, vec!["/info/refs?service=git-upload-pack", "/git-upload-pack"]);
    assert_eq!(storage.read("README").expect("work tree"), b"Hello World!\n");
}

#[test]
fn git_reads_what_the_clone_wrote() {
    if !git_available() {
        return;
    }
    let scratch = Scratch::new("remote_clone_git");
    let mut storage = scratch.storage();
    let mut transport = Replay::new(ADVERT, PACK);
    clone(&mut transport, &mut storage, ".git", "", "master", 1).expect("clone");

    git(&scratch.path, &["fsck", "--strict"]);
    assert_eq!(git(&scratch.path, &["rev-parse", "HEAD"]).trim(), HEAD);
    assert!(git(&scratch.path, &["status", "--porcelain"]).trim().is_empty());
}

#[test]
fn a_branch_the_remote_does_not_have_is_an_error() {
    let scratch = Scratch::new("remote_clone_missing");
    let mut storage = scratch.storage();
    let mut transport = Replay::new(ADVERT, PACK);
    assert!(clone(&mut transport, &mut storage, ".git", "", "nope", 1).is_err());
}
