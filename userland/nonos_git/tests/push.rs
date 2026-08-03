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
//! A push, delivered to a real `git receive-pack`.

mod common;

use common::{build_repo, git, git_available, receive_pack, Scratch};

use nonos_git::{objects_to_send, push_request, write_pack, ObjectId, RefUpdate};

const ZERO: &str = "0000000000000000000000000000000000000000";

#[test]
fn git_accepts_a_push_we_built() {
    if !git_available() {
        return;
    }
    let scratch = Scratch::new("push");
    let mut storage = scratch.storage();
    let (head, _blob) = build_repo(&mut storage);

    let objects = objects_to_send(&storage, ".git", &head, &[]).expect("objects");
    // A first commit carries itself, its tree and one blob.
    assert_eq!(objects.len(), 3);
    let pack = write_pack(&objects);

    let bare = scratch.path.join("remote.git");
    std::fs::create_dir_all(&bare).expect("remote dir");
    git(&bare, &["init", "--bare", "--quiet"]);

    let update = RefUpdate {
        old: ObjectId::from_hex(ZERO).expect("zero id"),
        new: head,
        name: "refs/heads/main",
    };
    let report = receive_pack(&bare, &push_request(&[update], &pack));

    assert!(report.contains("unpack ok"), "receive-pack said: {report}");
    assert!(report.contains("ok refs/heads/main"), "receive-pack said: {report}");
    assert_eq!(git(&bare, &["rev-parse", "refs/heads/main"]).trim(), head.to_hex());
    git(&bare, &["fsck", "--strict"]);
}
