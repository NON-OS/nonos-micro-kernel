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
//! A push driven end to end, with real git on the far end.

mod common;

use common::{build_repo, git, git_available, LocalGit, Scratch};

use nonos_git::push;

fn bare(scratch: &Scratch) -> std::path::PathBuf {
    let dir = scratch.path.join("remote.git");
    std::fs::create_dir_all(&dir).expect("remote dir");
    git(&dir, &["init", "--bare", "--quiet"]);
    dir
}

#[test]
fn a_push_moves_the_branch_on_the_remote() {
    if !git_available() {
        return;
    }
    let scratch = Scratch::new("remote_push");
    let mut storage = scratch.storage();
    let (head, _blob) = build_repo(&mut storage);
    let dir = bare(&scratch);

    let mut transport = LocalGit::new(&dir);
    push(&mut transport, &storage, ".git", &head, "refs/heads/main").expect("push");

    assert_eq!(git(&dir, &["rev-parse", "refs/heads/main"]).trim(), head.to_hex());
    git(&dir, &["fsck", "--strict"]);
}

#[test]
fn pushing_what_the_remote_already_has_sends_nothing() {
    if !git_available() {
        return;
    }
    let scratch = Scratch::new("remote_push_noop");
    let mut storage = scratch.storage();
    let (head, _blob) = build_repo(&mut storage);
    let dir = bare(&scratch);

    let mut transport = LocalGit::new(&dir);
    push(&mut transport, &storage, ".git", &head, "refs/heads/main").expect("first push");
    // The second push reads the advertisement, sees the ref already there and
    // stops. Sending an empty pack instead would be refused by the receiver.
    push(&mut transport, &storage, ".git", &head, "refs/heads/main").expect("second push");
    assert_eq!(git(&dir, &["rev-parse", "refs/heads/main"]).trim(), head.to_hex());
}
