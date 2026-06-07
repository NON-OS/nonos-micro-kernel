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
use super::packet::{parse, PACKET_LEN};
use super::ring::MouseRing;
pub struct MouseParser {
    buf: [u8; PACKET_LEN],
    index: usize,
}
impl MouseParser {
    pub const fn new() -> Self {
        Self { buf: [0; PACKET_LEN], index: 0 }
    }
    // Assemble 3-byte packets and queue them on the ring. Posting to the kernel
    // input ring is done once, by the pump loop draining this ring; absorbing
    // must not also post or every event reaches the kernel twice.
    pub fn absorb(&mut self, byte: u8, ring: &mut MouseRing) {
        if self.index == 0 && byte & 0x08 == 0 {
            ring.sync_errors = ring.sync_errors.wrapping_add(1);
            return;
        }
        self.buf[self.index] = byte;
        self.index += 1;
        if self.index == PACKET_LEN {
            self.index = 0;
            match parse(self.buf) {
                Some(ev) => ring.push(ev),
                None => ring.sync_errors = ring.sync_errors.wrapping_add(1),
            }
        }
    }
}
