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

//! Git reading the index and nested trees we staged.

mod common;

use common::{git, git_available, signature, Scratch};

use nonos_git::{add, commit, init, read_index, write_tree, CommitRequest, Mode, Storage};

#[test]
fn git_reads_the_index_and_commit_we_staged() {
    if !git_available() {
        eprintln!("skipping: no git on PATH");
        return;
    }

    let scratch = Scratch::new("stage_interop");
    let mut storage = scratch.storage();
    let root = scratch.path.clone();

    init(&mut storage, ".git", "main").expect("init");
    storage.write("top.txt", b"top\n").expect("work file");
    storage.write("dir/inner.txt", b"inner\n").expect("work file");
    for p in ["top.txt", "dir/inner.txt"] {
        add(&mut storage, ".git", p, Mode::File).expect("add");
    }

    let entries = read_index(&storage, ".git").expect("read index");
    let tree = write_tree(&mut storage, ".git", &entries).expect("write tree");
    let request = CommitRequest {
        tree,
        author: signature(),
        committer: signature(),
        message: String::from("staged\n"),
    };
    let head = commit(&mut storage, ".git", &request).expect("commit");

    // Git parses the index we wrote and lists exactly what we staged.
    let staged = git(&root, &["ls-files", "--stage"]);
    assert!(staged.contains("dir/inner.txt"), "index missing nested path: {staged}");
    assert!(staged.contains("top.txt"), "index missing top path: {staged}");

    git(&root, &["fsck", "--strict"]);
    assert_eq!(git(&root, &["rev-parse", "HEAD"]).trim(), head.to_hex());
    let tree_out = git(&root, &["ls-tree", "-r", "HEAD"]);
    assert!(tree_out.contains("dir/inner.txt"), "tree missing nested file: {tree_out}");

    // Index, commit and work tree all agree: nothing staged, modified or new.
    assert_eq!(git(&root, &["status", "--porcelain"]).trim(), "");
}
