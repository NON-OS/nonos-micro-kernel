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

//! The claim that matters: a repository this crate builds is one real git
//! reads. Git is never invoked to create any part of it.

mod common;

use std::fs;

use common::{build_repo, git, git_available, Scratch};

#[test]
fn git_reads_a_repository_we_built() {
    if !git_available() {
        eprintln!("skipping: no git on PATH");
        return;
    }

    let scratch = Scratch::new("interop");
    let mut storage = scratch.storage();
    let root = scratch.path.clone();
    let (head, blob) = build_repo(&mut storage);

    // fsck walks every object and ref and reports any corruption.
    git(&root, &["fsck", "--strict"]);
    assert_eq!(git(&root, &["rev-parse", "HEAD"]).trim(), head.to_hex());
    assert_eq!(git(&root, &["rev-parse", "--abbrev-ref", "HEAD"]).trim(), "main");

    let log = git(&root, &["log", "--pretty=%H %an <%ae> %s"]);
    assert!(log.contains(&head.to_hex()), "log missing our commit: {log}");
    assert!(log.contains("ek <ek@nonos.systems> first"), "log wrong: {log}");

    let ls = git(&root, &["ls-tree", "HEAD"]);
    assert_eq!(ls.trim(), format!("100644 blob {}\tf.txt", blob.to_hex()));
    assert_eq!(git(&root, &["cat-file", "blob", &blob.to_hex()]), "hello\n");

    // Git reads our tree and blob and writes the file back out. A plain
    // checkout cannot be used: we write no index, so it has nothing to
    // compare a pathspec against.
    git(&root, &["read-tree", "--reset", "-u", "HEAD"]);
    assert_eq!(fs::read_to_string(root.join("f.txt")).expect("checked out"), "hello\n");

    // With the index now written by git, it agrees the work tree matches.
    assert_eq!(git(&root, &["status", "--porcelain"]).trim(), "");
}
