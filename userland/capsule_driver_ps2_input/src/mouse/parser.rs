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
use super::packet::parse;
use super::ring::MouseRing;

const MAX_PACKET: usize = 4;

pub struct MouseParser {
    buf: [u8; MAX_PACKET],
    index: usize,
    // 3 for a plain mouse, 4 once IntelliMouse scroll is negotiated.
    packet_len: usize,
}
impl MouseParser {
    pub const fn new(wheel: bool) -> Self {
        Self { buf: [0; MAX_PACKET], index: 0, packet_len: if wheel { 4 } else { 3 } }
    }
    // Assemble one packet and queue it on the ring. Posting to the kernel input
    // ring is done once, by the pump loop draining this ring; absorbing must not
    // also post or every event reaches the kernel twice.
    pub fn absorb(&mut self, byte: u8, ring: &mut MouseRing) {
        if self.index == 0 && byte & 0x08 == 0 {
            ring.sync_errors = ring.sync_errors.wrapping_add(1);
            return;
        }
        self.buf[self.index] = byte;
        self.index += 1;
        if self.index == self.packet_len {
            self.index = 0;
            match parse(&self.buf[..self.packet_len]) {
                Some(ev) => ring.push(ev),
                None => ring.sync_errors = ring.sync_errors.wrapping_add(1),
            }
        }
    }
}
