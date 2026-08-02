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

//! Building a small repository the tests can assert against.

use nonos_git::{
    commit, encode_tree, init, write_object, CommitRequest, Mode, ObjectId, ObjectKind, Signature,
    TreeEntry,
};

use super::storage::DirStorage;

pub fn signature() -> Signature {
    Signature {
        name: String::from("ek"),
        email: String::from("ek@nonos.systems"),
        when: 1_700_000_000,
        offset_minutes: 0,
    }
}

/// A repository holding one file, returning the commit and blob ids.
pub fn build_repo(storage: &mut DirStorage) -> (ObjectId, ObjectId) {
    init(storage, ".git", "main").expect("init");
    let blob = write_object(storage, ".git", ObjectKind::Blob, b"hello\n").expect("blob");
    let mut entries = vec![TreeEntry { mode: Mode::File, name: String::from("f.txt"), id: blob }];
    let bytes = encode_tree(&mut entries);
    let tree = write_object(storage, ".git", ObjectKind::Tree, &bytes).expect("tree");
    let request = CommitRequest {
        tree,
        author: signature(),
        committer: signature(),
        message: String::from("first\n"),
    };
    (commit(storage, ".git", &request).expect("commit"), blob)
}
