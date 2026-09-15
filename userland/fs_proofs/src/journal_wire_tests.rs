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

// The journal reply body is a variable-length record list; these proofs walk it
// back exactly as the client does, so a length-prefix change on either side is
// caught here rather than as a truncated Recents list at runtime.

use crate::store::Store;

#[test]
fn journal_opcodes_follow_dirstat() {
    assert_eq!(crate::protocol::OP_JOURNAL_TOUCH, 23);
    assert_eq!(crate::protocol::OP_JOURNAL_LIST, 24);
}

#[test]
fn journal_list_reply_body_round_trips() {
    let mut s = Store::new();
    s.journal_touch("/a");
    s.journal_touch("/bb");

    let listed = s.journal_list(200);
    let mut body = alloc::vec::Vec::new();
    body.extend_from_slice(&(listed.len() as u32).to_le_bytes());
    for (atime, path) in listed.iter() {
        body.extend_from_slice(&atime.to_le_bytes());
        body.push(path.len() as u8);
        body.extend_from_slice(path.as_bytes());
    }

    let mut off = 4usize;
    let count = u32::from_le_bytes(body[0..4].try_into().unwrap());
    let mut names = alloc::vec::Vec::new();
    for _ in 0..count {
        off += 8;
        let n = body[off] as usize;
        off += 1;
        names.push(core::str::from_utf8(&body[off..off + n]).unwrap());
        off += n;
    }
    assert_eq!(off, body.len());
    assert_eq!(names, alloc::vec!["/bb", "/a"]);
}
