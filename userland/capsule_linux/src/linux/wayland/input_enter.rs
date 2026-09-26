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


//! The enter events a client needs before it will act on input.

use crate::linux::guest::Guest;

use super::out::Event;

const POINTER_ENTER: u16 = 0;
const KEYBOARD_ENTER: u16 = 1;

fn fixed(value: i32) -> u32 {
    (value << 8) as u32
}

pub fn enter_once(guest: &mut Guest, pointer: u32, x: i32, y: i32) {
    if guest.scene.pointer_entered {
        return;
    }
    let Some(surface) = guest.scene.surfaces.first().map(|s| s.id) else {
        return;
    };
    let serial = guest.scene.next_serial();
    Event::new(pointer, POINTER_ENTER)
        .u32(serial)
        .u32(surface)
        .u32(fixed(x))
        .u32(fixed(y))
        .send(&mut guest.display.to_client);
    guest.scene.pointer_entered = true;
}

/// The keys array is empty: nothing is held down at the moment a client is
/// told it has focus, and claiming otherwise would leave it with a key stuck
/// until the matching release it never saw.
pub fn keyboard_enter_once(guest: &mut Guest, keyboard: u32) {
    if guest.scene.keyboard_entered {
        return;
    }
    let Some(surface) = guest.scene.surfaces.first().map(|s| s.id) else {
        return;
    };
    let serial = guest.scene.next_serial();
    Event::new(keyboard, KEYBOARD_ENTER)
        .u32(serial)
        .u32(surface)
        .u32(0)
        .send(&mut guest.display.to_client);
    guest.scene.keyboard_entered = true;
}
