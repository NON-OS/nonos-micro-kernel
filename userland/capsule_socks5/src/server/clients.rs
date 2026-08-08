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

use alloc::vec::Vec;

use crate::conn::Conn;

/// Concurrent SOCKS handshakes. One page load opens several at once for the
/// document, styles, scripts and images, so a single slot cannot serve even
/// one client.
pub const MAX_CLIENTS: usize = 32;

struct Slot {
    pid: u32,
    conn: Conn,
}

/// Handshake state per caller, keyed on the pid the kernel attests at
/// delivery, so one capsule cannot drive another's handshake.
pub struct Clients {
    slots: Vec<Slot>,
}

impl Default for Clients {
    fn default() -> Self {
        Self::new()
    }
}

impl Clients {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    /// The handshake for `pid`, started if this is its first message. `None`
    /// when the table is full, which the caller answers by refusing the client
    /// rather than evicting somebody else's live session.
    pub fn get(&mut self, pid: u32) -> Option<&mut Conn> {
        if let Some(i) = self.slots.iter().position(|s| s.pid == pid) {
            return Some(&mut self.slots[i].conn);
        }
        if self.slots.len() == MAX_CLIENTS {
            return None;
        }
        self.slots.push(Slot { pid, conn: Conn::new() });
        self.slots.last_mut().map(|s| &mut s.conn)
    }

    /// Drop `pid`'s handshake, freeing its slot for the next caller.
    pub fn drop_client(&mut self, pid: u32) {
        self.slots.retain(|s| s.pid != pid);
    }
}
