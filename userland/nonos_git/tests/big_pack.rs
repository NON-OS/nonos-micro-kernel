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
//! Indexing a large pack, against real git's own listing.
//!
//! Set NONOS_GIT_BIG_PACK to a `.pack` to run these. They are skipped
//! otherwise, because a pack that size does not belong in the repository.

mod common;
mod verify_pack_row;

use common::git_available;
use verify_pack_row::git_row;

use nonos_git::{build_index_rows, pack_lookup, write_pack_index, Sha1};

#[test]
fn every_object_is_indexed_where_git_says_it_is() {
    let Ok(path) = std::env::var("NONOS_GIT_BIG_PACK") else {
        return;
    };
    if !git_available() {
        return;
    }
    let pack = std::fs::read(&path).expect("pack file");
    let rows = build_index_rows(&pack).expect("index rows");

    let listing = std::process::Command::new("git")
        .args(["verify-pack", "-v", &path.replace(".pack", ".idx")])
        .output()
        .expect("verify-pack");
    let text = String::from_utf8_lossy(&listing.stdout);

    let mut checked = 0usize;
    for (id, at) in text.lines().filter_map(git_row) {
        let ours = rows.iter().find(|(r, _, _)| *r == id).expect("git listed an id we missed");
        assert_eq!(ours.1, at, "offset for {}", id.to_hex());
        checked += 1;
    }
    assert!(checked > 1000, "expected a large pack, checked {checked}");

    // The index we write has to find every one of them back.
    let sha = Sha1::digest(&pack[..pack.len() - 20]);
    let idx = write_pack_index(&rows, &sha).expect("index");
    for (id, at, _) in &rows {
        assert_eq!(pack_lookup(&idx, id), Some(*at));
    }
}

/// Indexing on its own, with nothing else held, so a memory measurement of
/// this is what a clone of that size would actually cost.
#[test]
fn indexing_a_large_pack_alone() {
    let Ok(path) = std::env::var("NONOS_GIT_BIG_PACK") else {
        return;
    };
    let pack = std::fs::read(&path).expect("pack file");
    let rows = build_index_rows(&pack).expect("index rows");
    println!("pack {} bytes, {} objects", pack.len(), rows.len());
}
