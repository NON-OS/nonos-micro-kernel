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

/// The most concurrent tunneled connections.
pub const MAX_CONNS: usize = 64;

#[derive(Clone, Copy)]
pub(super) struct Slot {
    pub(super) id: u64,
    pub(super) socket: u32,
    /// The next stream position to stamp on a send for this connection. Nym send
    /// requests carry a sequence so the exit can reassemble a reordered stream.
    pub(super) seq: u64,
    pub(super) used: bool,
}

/// The live connection table.
pub struct Manager {
    pub(super) slots: [Slot; MAX_CONNS],
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
}
