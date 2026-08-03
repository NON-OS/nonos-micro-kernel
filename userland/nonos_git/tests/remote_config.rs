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
//! Remotes, written so git reads them back as its own.

mod common;

use common::{git, git_available, Scratch};

use nonos_git::{init, remote_url, set_remote, Storage};

const URL: &str = "https://github.com/octocat/Hello-World.git";

#[test]
fn a_remote_survives_a_round_trip() {
    let scratch = Scratch::new("remote_config");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");

    assert_eq!(remote_url(&storage, ".git", "origin"), None);
    set_remote(&mut storage, ".git", "origin", URL).expect("set");
    assert_eq!(remote_url(&storage, ".git", "origin").as_deref(), Some(URL));
}

#[test]
fn setting_a_remote_twice_leaves_one_section() {
    let scratch = Scratch::new("remote_twice");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");

    set_remote(&mut storage, ".git", "origin", URL).expect("first");
    set_remote(&mut storage, ".git", "origin", "https://example.com/x.git").expect("second");

    let text = String::from_utf8(storage.read(".git/config").expect("config")).expect("utf8");
    assert_eq!(text.matches("[remote \"origin\"]").count(), 1);
    assert_eq!(
        remote_url(&storage, ".git", "origin").as_deref(),
        Some("https://example.com/x.git")
    );
    // The core section the repository was created with has to survive.
    assert!(text.contains("repositoryformatversion"));
}

#[test]
fn git_reads_the_remote_we_wrote() {
    if !git_available() {
        return;
    }
    let scratch = Scratch::new("remote_git");
    let mut storage = scratch.storage();
    init(&mut storage, ".git", "main").expect("init");
    set_remote(&mut storage, ".git", "origin", URL).expect("set");

    assert_eq!(git(&scratch.path, &["remote", "get-url", "origin"]).trim(), URL);
    assert_eq!(git(&scratch.path, &["config", "--get", "remote.origin.url"]).trim(), URL);
}
