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


//! The three requests a client makes before it asks for anything real.

use crate::linux::guest::Guest;

use super::object::Object;
use super::out::Event;
use super::registry::{by_name, GLOBALS};
use super::args::Args;

const CALLBACK_DONE: u16 = 0;
const REGISTRY_GLOBAL: u16 = 0;
const SEAT_CAPABILITIES: u16 = 0;
/// A pointer and a keyboard, which is what the input events carry.
const SEAT_POINTER_AND_KEYBOARD: u32 = 3;

/// Nothing here is asynchronous, so a sync is done the moment it is
/// asked for and the callback fires in the same batch.
pub fn sync(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    Event::new(id, CALLBACK_DONE).u32(0).send(&mut guest.display.to_client);
}

pub fn registry(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    guest.objects.put(id, Object::Registry);
    for global in GLOBALS {
        Event::new(id, REGISTRY_GLOBAL)
            .u32(global.name)
            .string(global.interface)
            .u32(global.version)
            .send(&mut guest.display.to_client);
    }
}

fn seat_caps(guest: &mut Guest, id: u32) {
    Event::new(id, SEAT_CAPABILITIES)
        .u32(SEAT_POINTER_AND_KEYBOARD)
        .send(&mut guest.display.to_client);
}

pub fn bind(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(name), Some(_iface), Some(_ver), Some(id)) =
        (args.u32(), args.string(), args.u32(), args.u32())
    else {
        return;
    };
    let Some(global) = by_name(name) else { return };
    guest.objects.put(id, global.object);
    match global.object {
        /*
         * A client reads the format list before it asks for a pool, and
         * a seat's capabilities before it asks for a keyboard. Both are
         */
        Object::Shm => crate::linux::wayland::shm::formats(guest, id),
        Object::Seat => seat_caps(guest, id),
        _ => {}
    }
}
