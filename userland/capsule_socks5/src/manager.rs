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

//! Track the tunneled connections. One mixnet session carries every client's
//! traffic, so each SOCKS5 client that reaches the relay phase is given a
//! connection id that its tunnel frames carry. A response arriving off the mixnet
//! names an id; the manager maps it back to the client socket to write, and the
//! reverse (a client socket closing) back to the id to tear down. Fixed capacity,
//! no allocation: the server refuses a new client rather than growing unbounded.

/// The most concurrent tunneled connections.
pub const MAX_CONNS: usize = 64;

#[derive(Clone, Copy)]
struct Slot {
    id: u64,
    socket: u32,
    /// The next stream position to stamp on a send for this connection. Nym send
    /// requests carry a sequence so the exit can reassemble a reordered stream.
    seq: u64,
    used: bool,
}

/// The live connection table.
pub struct Manager {
    slots: [Slot; MAX_CONNS],
    next_id: u64,
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager {
    pub fn new() -> Self {
        Self { slots: [Slot { id: 0, socket: 0, seq: 0, used: false }; MAX_CONNS], next_id: 1 }
    }

    /// Register `socket` and return the connection id assigned to it, or `None`
    /// when the table is full. Ids never repeat within a session and never take
    /// the value zero (reserved as "no connection").
    pub fn open(&mut self, socket: u32) -> Option<u64> {
        let i = self.slots.iter().position(|s| !s.used)?;
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        if self.next_id == 0 {
            self.next_id = 1;
        }
        self.slots[i] = Slot { id, socket, seq: 0, used: true };
        Some(id)
    }

    /// The next send sequence for `id`, advancing its counter, or `None` if the
    /// connection is unknown or closed. The first send on a connection is
    /// sequence zero.
    pub fn next_seq(&mut self, id: u64) -> Option<u64> {
        let slot = self.slots.iter_mut().find(|s| s.used && s.id == id)?;
        let seq = slot.seq;
        slot.seq = slot.seq.wrapping_add(1);
        Some(seq)
    }

    /// The client socket for `id`, or `None` if it is unknown or closed.
    pub fn socket_of(&self, id: u64) -> Option<u32> {
        self.slots.iter().find(|s| s.used && s.id == id).map(|s| s.socket)
    }

    /// Close the connection named by `id`, returning the client socket to close.
    pub fn close(&mut self, id: u64) -> Option<u32> {
        let slot = self.slots.iter_mut().find(|s| s.used && s.id == id)?;
        slot.used = false;
        Some(slot.socket)
    }

    /// Close the connection whose client socket is `socket` (the client hung up),
    /// returning the id so its tunnel can be closed.
    pub fn close_socket(&mut self, socket: u32) -> Option<u64> {
        let slot = self.slots.iter_mut().find(|s| s.used && s.socket == socket)?;
        slot.used = false;
        Some(slot.id)
    }

    /// How many connections are live.
    pub fn count(&self) -> usize {
        self.slots.iter().filter(|s| s.used).count()
    }
}
