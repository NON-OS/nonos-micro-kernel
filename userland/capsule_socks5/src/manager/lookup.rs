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

use super::table::Manager;

impl Manager {
    /// The client socket for `id`, or `None` if unknown or closed.
    pub fn socket_of(&self, id: u64) -> Option<u32> {
        self.slots.iter().find(|s| s.used && s.id == id).map(|s| s.socket)
    }

    /// Close the connection named by `id`, returning the client socket to
    /// close.
    pub fn close(&mut self, id: u64) -> Option<u32> {
        let slot = self.slots.iter_mut().find(|s| s.used && s.id == id)?;
        slot.used = false;
        Some(slot.socket)
    }

    /// Close the connection whose client socket is `socket`, the client having
    /// hung up, returning the id so its tunnel can be closed too.
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
