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

use crate::files_fixture::{put, SEARCH_CONTENT, SEARCH_NAMES};
use crate::store::Store;

#[test]
fn search_opcode_follows_the_journal_pair() {
    assert_eq!(crate::protocol::OP_SEARCH, 25);
}

#[test]
fn search_reply_body_round_trips() {
    let mut s = Store::new();
    put(&mut s, "/needle.txt", b"x");
    put(&mut s, "/hay.txt", b"a\nneedle\n");

    let hits = s.search("needle", SEARCH_NAMES | SEARCH_CONTENT, 200);
    let mut body = alloc::vec::Vec::new();
    body.extend_from_slice(&(hits.len() as u32).to_le_bytes());
    for (kind, line, path) in hits.iter() {
        body.extend_from_slice(&kind.to_le_bytes());
        body.extend_from_slice(&line.to_le_bytes());
        body.push(path.len() as u8);
        body.extend_from_slice(path.as_bytes());
    }

    let count = u32::from_le_bytes(body[0..4].try_into().unwrap());
    assert_eq!(count, 2);
    let mut off = 4usize;
    let mut decoded = alloc::vec::Vec::new();
    for _ in 0..count {
        let kind = u32::from_le_bytes(body[off..off + 4].try_into().unwrap());
        let line = u32::from_le_bytes(body[off + 4..off + 8].try_into().unwrap());
        off += 8;
        let n = body[off] as usize;
        off += 1;
        decoded.push((kind, line, core::str::from_utf8(&body[off..off + n]).unwrap()));
        off += n;
    }
    assert_eq!(off, body.len());
    assert_eq!(decoded, alloc::vec![(0, 0, "/needle.txt"), (1, 2, "/hay.txt")]);
}
