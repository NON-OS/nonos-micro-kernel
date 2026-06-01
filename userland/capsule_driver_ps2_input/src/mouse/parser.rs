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
use super::post;
use super::ring::MouseRing;
pub struct MouseParser {
    buf: [u8; PACKET_LEN],
    index: usize,
    previous_buttons: u8,
}
impl MouseParser {
    pub const fn new() -> Self {
        Self { buf: [0; PACKET_LEN], index: 0, previous_buttons: 0 }
    }
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
                Some(ev) => {
                    let previous = self.previous_buttons;
                    self.previous_buttons = ev.buttons;
                    ring.push(ev);
                    if !post::publish(ev, previous) {
                        ring.sync_errors = ring.sync_errors.wrapping_add(1);
                    }
                }
                None => ring.sync_errors = ring.sync_errors.wrapping_add(1),
            }
        }
    }
}
