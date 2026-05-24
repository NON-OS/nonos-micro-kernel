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
use super::drainer::Drainer;
use crate::keymap;
use crate::ring::{Event, Ring, FLAG_BREAK, FLAG_E0_PREFIX, FLAG_E1_PREFIX};
const E0_PREFIX: u8 = 0xE0;
const E1_PREFIX: u8 = 0xE1;
const BREAK_BIT: u8 = 0x80;
pub(super) fn absorb(drainer: &mut Drainer, ring: &mut Ring, byte: u8) {
    if byte == E0_PREFIX {
        drainer.pending_e0 = true;
        return;
    }
    if byte == E1_PREFIX {
        drainer.pending_e1 = true;
        return;
    }
    let mut flags: u8 = 0;
    if byte & BREAK_BIT != 0 {
        flags |= FLAG_BREAK;
    }
    if drainer.pending_e0 {
        flags |= FLAG_E0_PREFIX;
        drainer.pending_e0 = false;
    }
    if drainer.pending_e1 {
        flags |= FLAG_E1_PREFIX;
        drainer.pending_e1 = false;
    }
    ring.push(Event { scancode: byte, flags });
    if let Some(t) = keymap::translate(byte, flags) {
        let _ = keymap::publish(t);
    }
}