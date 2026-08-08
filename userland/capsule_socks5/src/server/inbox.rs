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

extern crate alloc;

use alloc::vec::Vec;

/// A chunk that arrived before the bytes in front of it.
struct Held {
    conn: u64,
    seq: u64,
    closed: bool,
    data: Vec<u8>,
}

/// Stream messages coming back, put back in order.
///
/// The mixnet delays every packet on purpose, so pieces of one stream arrive
/// in whatever order their delays happened to produce. Handing them on as
/// they land would corrupt the stream, and a page assembled out of order
/// fails in ways that look like anything but the network.
///
/// The far end numbers each message it sends, counting messages rather than
/// bytes, and expects them read back in that order.
#[derive(Default)]
pub struct Inbox {
    held: Vec<Held>,
    /// Next message number expected per connection.
    marks: Vec<(u64, u64)>,
}

impl Inbox {
    pub fn accept(&mut self, conn: u64, seq: u64, closed: bool, data: &[u8]) {
        self.held.push(Held { conn, seq, closed, data: data.to_vec() });
    }

    /// Take every chunk of `conn` that continues the stream, in order.
    /// Returns the bytes and whether the far end finished.
    pub fn drain(&mut self, conn: u64) -> (Vec<u8>, bool) {
        let mut out = Vec::new();
        let mut done = false;
        loop {
            let want = self.mark(conn);
            let Some(i) = self.held.iter().position(|h| h.conn == conn && h.seq == want) else {
                break;
            };
            let chunk = self.held.remove(i);
            out.extend_from_slice(&chunk.data);
            done |= chunk.closed;
            // Positions count messages, not bytes: the far end numbers each
            // one and expects them read back in that order. Advancing by the
            // length of what arrived asks for a position nothing will ever
            // carry, and the stream stops at the first chunk.
            self.set_mark(conn, want.wrapping_add(1));
        }
        (out, done)
    }

    /// Forget a connection, along with anything still held for it.
    pub fn forget(&mut self, conn: u64) {
        self.held.retain(|h| h.conn != conn);
        self.marks.retain(|m| m.0 != conn);
    }

    fn mark(&self, conn: u64) -> u64 {
        self.marks.iter().find(|m| m.0 == conn).map(|m| m.1).unwrap_or(0)
    }

    fn set_mark(&mut self, conn: u64, at: u64) {
        match self.marks.iter_mut().find(|m| m.0 == conn) {
            Some(m) => m.1 = at,
            None => self.marks.push((conn, at)),
        }
    }
}
