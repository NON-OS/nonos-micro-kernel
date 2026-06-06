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
use super::event::Event;
use super::state::Ring;
use crate::constants::RING_CAPACITY;
impl Ring {
    pub fn push(&mut self, ev: Event) {
        let next = (self.head + 1) % RING_CAPACITY;
        if next == self.tail {
            self.tail = (self.tail + 1) % RING_CAPACITY;
            self.events_dropped = self.events_dropped.wrapping_add(1);
        }
        self.buf[self.head] = ev;
        self.head = next;
        self.events_seen = self.events_seen.wrapping_add(1);
    }
}
