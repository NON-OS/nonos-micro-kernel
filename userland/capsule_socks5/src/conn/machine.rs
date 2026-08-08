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

use super::event::{Event, Phase};

/// The most bytes a handshake accumulates: a request with a full 255-byte
/// domain (`ver+cmd+rsv+atyp+len` = 5, domain 255, port 2).
pub(super) const ACC_MAX: usize = 262;

/// A SOCKS5 connection mid-handshake.
pub struct Conn {
    pub(super) phase: Phase,
    pub(super) acc: [u8; ACC_MAX],
    pub(super) len: usize,
}

impl Default for Conn {
    fn default() -> Self {
        Self::new()
    }
}

impl Conn {
    pub fn new() -> Self {
        Self { phase: Phase::Greeting, acc: [0u8; ACC_MAX], len: 0 }
    }

    pub fn is_relaying(&self) -> bool {
        self.phase == Phase::Relaying
    }

    pub fn is_closed(&self) -> bool {
        self.phase == Phase::Closed
    }

    /// Feed handshake bytes and advance. A client flooding past the
    /// accumulator closes the connection rather than growing it.
    pub fn on_client(&mut self, data: &[u8]) -> Event {
        // A relaying connection has no handshake left to accumulate. Its
        // bytes belong to the tunnel, and holding them here would both
        // corrupt the stream and fill a buffer sized for a handshake.
        if self.phase == Phase::Relaying {
            return Event::Relay;
        }
        let room = ACC_MAX - self.len;
        if data.len() > room {
            self.phase = Phase::Closed;
            return Event::Close;
        }
        self.acc[self.len..self.len + data.len()].copy_from_slice(data);
        self.len += data.len();

        match self.phase {
            Phase::Greeting => self.greeting(),
            Phase::Request => self.request(),
            Phase::Relaying | Phase::Closed => Event::Close,
        }
    }

    // Keep bytes belonging to the next phase: a client may pipeline the
    // request behind the greeting.
    pub(super) fn drain(&mut self, n: usize) {
        let n = n.min(self.len);
        self.acc.copy_within(n..self.len, 0);
        self.len -= n;
    }
}
