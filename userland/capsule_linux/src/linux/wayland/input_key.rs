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


//! A key event, written to the client.

use crate::linux::guest::Guest;

use super::input_enter::keyboard_enter_once;
use super::out::Event;

/// wl_keyboard: key is 3.
const KEYBOARD_KEY: u16 = 3;

pub fn key(guest: &mut Guest, code: u32, state: u32) {
    let Some(id) = guest.scene.keyboard else { return };
    keyboard_enter_once(guest, id);
    let serial = guest.scene.next_serial();
    let time = time_ms();
    Event::new(id, KEYBOARD_KEY)
        .u32(serial)
        .u32(time)
        .u32(code)
        .u32(state)
        .send(&mut guest.display.to_client);
}

fn time_ms() -> u32 {
    nonos_libc::mk_uptime_ms().max(0) as u32
}
