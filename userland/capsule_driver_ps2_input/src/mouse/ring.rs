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
use super::event::MouseEvent;
pub const MOUSE_RING_CAPACITY: usize = 128;
const EMPTY: MouseEvent = MouseEvent { dx: 0, dy: 0, dz: 0, buttons: 0, flags: 0 };
pub struct MouseRing {
    buf: [MouseEvent; MOUSE_RING_CAPACITY],
    head: usize,
    tail: usize,
    pub events_seen: u64,
    pub events_dropped: u64,
    pub sync_errors: u64,
}
impl MouseRing {
    pub const fn new() -> Self {
        Self {
            buf: [EMPTY; MOUSE_RING_CAPACITY],
            head: 0,
            tail: 0,
            events_seen: 0,
            events_dropped: 0,
            sync_errors: 0,
        }
    }
    pub fn push(&mut self, ev: MouseEvent) {
        self.events_seen = self.events_seen.wrapping_add(1);
        let next = (self.tail + 1) % MOUSE_RING_CAPACITY;
        if next == self.head {
            self.events_dropped = self.events_dropped.wrapping_add(1);
            return;
        }
        self.buf[self.tail] = ev;
        self.tail = next;
    }
    pub fn pop(&mut self) -> Option<MouseEvent> {
        if self.head == self.tail {
            return None;
        }
        let ev = self.buf[self.head];
        self.head = (self.head + 1) % MOUSE_RING_CAPACITY;
        Some(ev)
    }
    pub fn queued(&self) -> usize {
        if self.tail >= self.head {
            self.tail - self.head
        } else {
            MOUSE_RING_CAPACITY - self.head + self.tail
        }
    }
}
