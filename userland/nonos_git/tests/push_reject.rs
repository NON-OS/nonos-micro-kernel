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
//! Pushes a receiver must refuse.

mod common;

use common::{build_repo, git, git_available, receive_pack, Scratch};

use nonos_git::{objects_to_send, push_request, write_pack, ObjectId, RefUpdate};

const ZERO: &str = "0000000000000000000000000000000000000000";

/// A bare repository plus the pack and head of a fresh one-commit repo.
fn fixture(tag: &str) -> (Scratch, std::path::PathBuf, ObjectId, Vec<u8>) {
    let scratch = Scratch::new(tag);
    let mut storage = scratch.storage();
    let (head, _blob) = build_repo(&mut storage);
    let pack = write_pack(&objects_to_send(&storage, ".git", &head, &[]).expect("objects"));
    let bare = scratch.path.join("remote.git");
    std::fs::create_dir_all(&bare).expect("remote dir");
    git(&bare, &["init", "--bare", "--quiet"]);
    (scratch, bare, head, pack)
}

fn update(head: ObjectId) -> RefUpdate<'static> {
    RefUpdate { old: ObjectId::from_hex(ZERO).expect("zero"), new: head, name: "refs/heads/main" }
}

#[test]
fn a_damaged_pack_is_not_unpacked() {
    if !git_available() {
        return;
    }
    let (_scratch, bare, head, mut pack) = fixture("push_damaged");
    // Flip a bit inside the first object's zlib stream, past the header.
    pack[20] ^= 0x40;

    let report = receive_pack(&bare, &push_request(&[update(head)], &pack));
    assert!(!report.contains("unpack ok"), "damage went unnoticed: {report}");
}

#[test]
fn a_stale_old_id_is_refused() {
    if !git_available() {
        return;
    }
    let (_scratch, bare, head, pack) = fixture("push_stale");
    let report = receive_pack(&bare, &push_request(&[update(head)], &pack));
    assert!(report.contains("ok refs/heads/main"), "first push failed: {report}");

    // The ref now holds head, so a second push still claiming it is unborn is
    // out of date and must not be applied.
    let again = receive_pack(&bare, &push_request(&[update(head)], &pack));
    assert!(again.contains("ng refs/heads/main"), "stale push accepted: {again}");
}
