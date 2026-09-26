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


//! One connection: what the client has said, and what it has not read.

use alloc::vec::Vec;

pub struct Conn {
    /// Bytes the guest wrote and the server has not consumed.
    pub to_server: Vec<u8>,
    /// Bytes the server produced and the guest has not read.
    pub to_client: Vec<u8>,
    /// Descriptors the client passed in control data, in order.
    pub fds: Vec<u32>,
    /// Descriptors owed to the client, for the next control block.
    pub give: Vec<u32>,
}

impl Conn {
    pub fn new() -> Conn {
        Conn { to_server: Vec::new(), to_client: Vec::new(), fds: Vec::new(), give: Vec::new() }
    }

    /// Take everything the server has produced, up to `want`.
    pub fn drain(&mut self, want: usize) -> Vec<u8> {
        let n = want.min(self.to_client.len());
        self.to_client.drain(..n).collect()
    }
}

impl Default for Conn {
    fn default() -> Conn {
        Conn::new()
    }
}
