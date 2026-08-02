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
//! reads.
//!
//! Everything else checks our bytes against ids captured from git. This runs
//! `git` itself over a repository created entirely by this code, with no git
//! involvement in making it, and asserts git reports the commit, the tree and
//! the file contents we recorded. If our object framing, zlib, tree sort, ref
//! layout or commit encoding were wrong in any way git cares about, `fsck` and
//! `log` would say so here.
//!
//! The test skips itself when no `git` is on PATH, so the suite still runs in
//! an environment without one rather than reporting a failure it cannot judge.

mod common;

use std::fs;
use std::path::Path;
use std::process::Command;

use common::Scratch;

use nonos_git::{
    commit, encode_tree, init, write_object, CommitRequest, Mode, ObjectKind, Signature, TreeEntry,
};

fn git_available() -> bool {
    Command::new("git").arg("--version").output().map(|o| o.status.success()).unwrap_or(false)
}

/// Run git in `dir` and return its stdout, asserting it succeeded.
fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("running git {args:?}: {e}"));
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn git_reads_a_repository_we_built() {
    if !git_available() {
        eprintln!("skipping: no git on PATH");
        return;
    }

    let scratch = Scratch::new("interop");
    let mut storage = scratch.storage();
    let root = scratch.path.clone();

    // Build the repository with this crate only; git is never invoked to
    // create any part of it.
    init(&mut storage, ".git", "main").expect("init");
    let blob = write_object(&mut storage, ".git", ObjectKind::Blob, b"hello\n").expect("blob");
    let mut entries = vec![TreeEntry { mode: Mode::File, name: String::from("f.txt"), id: blob }];
    let tree_bytes = encode_tree(&mut entries);
    let tree = write_object(&mut storage, ".git", ObjectKind::Tree, &tree_bytes).expect("tree");

    let sig = Signature {
        name: String::from("ek"),
        email: String::from("ek@nonos.systems"),
        when: 1_700_000_000,
        offset_minutes: 0,
    };
    let request = CommitRequest {
        tree,
        author: sig.clone(),
        committer: sig,
        message: String::from("first\n"),
    };
    let head = commit(&mut storage, ".git", &request).expect("commit");

    // Now let git judge it.

    // fsck walks every object and ref and reports any corruption.
    git(&root, &["fsck", "--strict"]);

    // The commit git resolves for HEAD is the one we wrote.
    assert_eq!(git(&root, &["rev-parse", "HEAD"]).trim(), head.to_hex());

    // The branch git sees is the one init created.
    assert_eq!(git(&root, &["rev-parse", "--abbrev-ref", "HEAD"]).trim(), "main");

    // The log shows our message and author.
    let log = git(&root, &["log", "--pretty=%H %an <%ae> %s"]);
    assert!(log.contains(&head.to_hex()), "log missing our commit: {log}");
    assert!(log.contains("ek <ek@nonos.systems> first"), "log wrong: {log}");

    // The tree git reads holds the file we recorded, with the mode and blob id.
    let ls = git(&root, &["ls-tree", "HEAD"]);
    assert_eq!(ls.trim(), format!("100644 blob {}\tf.txt", blob.to_hex()));

    // And the blob's bytes come back through git's own object reader.
    assert_eq!(git(&root, &["cat-file", "blob", &blob.to_hex()]), "hello\n");

    // Populate the index and work tree from our commit. This is the strongest
    // check: git reads our tree and blob and writes the file back out. A plain
    // `checkout` cannot be used because we deliberately write no index, so
    // there is nothing for it to compare a pathspec against.
    git(&root, &["read-tree", "--reset", "-u", "HEAD"]);
    assert_eq!(fs::read_to_string(root.join("f.txt")).expect("checked out file"), "hello\n");

    // With the index now written by git, it agrees the work tree matches the
    // commit: nothing modified, nothing untracked.
    assert_eq!(git(&root, &["status", "--porcelain"]).trim(), "");
}
