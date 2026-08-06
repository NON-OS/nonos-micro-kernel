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
    /// The connection id opened for `socket`, or `None` if it has none.
    pub fn id_of_socket(&self, socket: u32) -> Option<u64> {
        self.slots.iter().find(|s| s.used && s.socket == socket).map(|s| s.id)
    }

    /// Claim the stream position for a send of `len` bytes, advancing the
    /// connection past it.
    ///
    /// The exit reassembles on these, so every byte sent has to be counted
    /// exactly once even when the send that carried it fails. Reusing a
    /// position would have the exit take the retry as the same bytes arriving
    /// twice and drop one of them.
    pub fn take_seq(&mut self, id: u64, len: usize) -> Option<u64> {
        let slot = self.slots.iter_mut().find(|s| s.used && s.id == id)?;
        let at = slot.seq;
        slot.seq = slot.seq.wrapping_add(len as u64);
        Some(at)
    }
}
