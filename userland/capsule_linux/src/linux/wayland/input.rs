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


//! NONOS input events, as Wayland sees them.

use nonos_libc::{mk_input_event_drain, InputEvent};

use crate::linux::guest::Guest;

use super::input_key::key;
use super::input_send::{button, motion};

/// Events taken in one pass. A deeper backlog is drained on the next.
const BATCH: usize = 32;

const KEY_DOWN: u16 = 0;
const KEY_UP: u16 = 1;
const POINTER_ABS: u16 = 3;
const BUTTON_DOWN: u16 = 5;
const BUTTON_UP: u16 = 6;

pub fn pump(guest: &mut Guest) {
    if guest.scene.pointer.is_none() && guest.scene.keyboard.is_none() {
        return;
    }
    let mut events = [InputEvent::default(); BATCH];
    let n = mk_input_event_drain(events.as_mut_ptr(), BATCH as u64);
    if n <= 0 {
        return;
    }
    for event in events.iter().take(n as usize) {
        deliver(guest, event);
    }
}

fn deliver(guest: &mut Guest, event: &InputEvent) {
    match event.kind {
        KEY_DOWN => key(guest, event.code, 1),
        KEY_UP => key(guest, event.code, 0),
        POINTER_ABS => motion(guest, event.x, event.y),
        BUTTON_DOWN => button(guest, event.code, 1),
        BUTTON_UP => button(guest, event.code, 0),
        _ => {}
    }
}
