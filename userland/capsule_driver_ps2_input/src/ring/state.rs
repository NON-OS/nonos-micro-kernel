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
use crate::constants::RING_CAPACITY;
pub struct Ring {
    pub(super) buf: [Event; RING_CAPACITY],
    pub(super) head: usize,
    pub(super) tail: usize,
    pub events_seen: u64,
    pub events_dropped: u64,
    pub parity_errors: u64,
    pub timeout_errors: u64,
}
impl Ring {
    pub const fn new() -> Self {
        Self {
            buf: [Event { scancode: 0, flags: 0 }; RING_CAPACITY],
            head: 0,
            tail: 0,
            events_seen: 0,
            events_dropped: 0,
            parity_errors: 0,
            timeout_errors: 0,
        }
    }
    pub fn queued(&self) -> usize {
        if self.tail >= self.head {
            self.tail - self.head
        } else {
            RING_CAPACITY - self.head + self.tail
        }
    }
    pub fn head(&self) -> usize {
        self.head
    }
    pub fn tail(&self) -> usize {
        self.tail
    }
}